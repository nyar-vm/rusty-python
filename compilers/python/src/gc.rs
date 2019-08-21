//! Python 垃圾回收机制实现
//!
//! 实现引用计数 + 分代标记-清除的混合垃圾回收算法

use python_types::PythonValue;
use std::sync::{Arc, Mutex};

/// 代级别
#[derive(Debug, Clone, PartialEq)]
pub enum Generation {
    /// 年轻代
    Young,
    /// 老年代
    Old,
}

/// 带代信息的对象
#[derive(Clone)]
pub struct GcObject {
    /// 对象值
    value: Arc<PythonValue>,
    /// 代级别
    generation: Generation,
    /// 年龄（经过的垃圾回收次数）
    age: usize,
}

/// 垃圾回收器
pub struct GC {
    /// 根对象集合
    roots: Vec<Arc<PythonValue>>,
    /// 年轻代对象
    young_objects: Vec<GcObject>,
    /// 老年代对象
    old_objects: Vec<GcObject>,
    /// 运行状态
    running: bool,
    /// 年轻代垃圾回收阈值
    young_threshold: usize,
    /// 老年代垃圾回收阈值
    old_threshold: usize,
    /// 年轻代垃圾回收次数
    young_collections: usize,
    /// 老年代垃圾回收次数
    old_collections: usize,
}

impl GC {
    /// 创建新的垃圾回收器
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            young_objects: Vec::new(),
            old_objects: Vec::new(),
            running: true,
            young_threshold: 1000,
            old_threshold: 10000,
            young_collections: 0,
            old_collections: 0,
        }
    }

    /// 注册根对象
    pub fn register_root(&mut self, value: Arc<PythonValue>) {
        self.roots.push(value.clone());
        self.young_objects.push(GcObject { value: value, generation: Generation::Young, age: 0 });
    }

    /// 注册对象
    pub fn register_object(&mut self, value: Arc<PythonValue>) {
        self.young_objects.push(GcObject { value: value, generation: Generation::Young, age: 0 });
    }

    /// 执行垃圾回收
    pub fn collect(&mut self) {
        // 先执行年轻代垃圾回收
        self.collect_young();

        // 如果年轻代垃圾回收次数达到阈值，执行老年代垃圾回收
        if self.young_collections % 10 == 0 {
            self.collect_old();
        }
    }

    /// 执行年轻代垃圾回收
    fn collect_young(&mut self) {
        // 标记阶段
        let mut marked = vec![false; self.young_objects.len()];
        self.mark_roots_young(&mut marked);

        // 清除阶段
        self.sweep_young(&marked);

        self.young_collections += 1;
    }

    /// 执行老年代垃圾回收
    fn collect_old(&mut self) {
        // 标记阶段
        let mut marked = vec![false; self.old_objects.len()];
        self.mark_roots_old(&mut marked);

        // 清除阶段
        self.sweep_old(&marked);

        self.old_collections += 1;
    }

    /// 标记根对象及其引用的年轻代对象
    fn mark_roots_young(&self, marked: &mut Vec<bool>) {
        for root in &self.roots {
            self.mark_object_young(root, marked);
        }
    }

    /// 标记根对象及其引用的老年代对象
    fn mark_roots_old(&self, marked: &mut Vec<bool>) {
        for root in &self.roots {
            self.mark_object_old(root, marked);
        }
    }

    /// 标记年轻代对象及其引用的对象
    fn mark_object_young(&self, value: &Arc<PythonValue>, marked: &mut Vec<bool>) {
        // 查找对象在 young_objects 中的索引
        if let Some(index) = self.young_objects.iter().position(|obj| Arc::ptr_eq(&obj.value, value)) {
            if !marked[index] {
                marked[index] = true;

                // 递归标记引用的对象
                match value.as_ref() {
                    PythonValue::List(list) => {
                        for item in list {
                            self.mark_object_young(item, marked);
                            self.mark_object_old(item, &mut vec![false; self.old_objects.len()]);
                        }
                    }
                    PythonValue::Tuple(tuple) => {
                        for item in tuple {
                            self.mark_object_young(item, marked);
                            self.mark_object_old(item, &mut vec![false; self.old_objects.len()]);
                        }
                    }
                    PythonValue::Dict(dict) => {
                        for (_, value) in dict {
                            self.mark_object_young(value, marked);
                            self.mark_object_old(value, &mut vec![false; self.old_objects.len()]);
                        }
                    }
                    PythonValue::Object(_, attrs) => {
                        for (_, value) in attrs {
                            self.mark_object_young(value, marked);
                            self.mark_object_old(value, &mut vec![false; self.old_objects.len()]);
                        }
                    }
                    _ => {
                        // 基本类型，不需要标记
                    }
                }
            }
        }
    }

    /// 标记老年代对象及其引用的对象
    fn mark_object_old(&self, value: &Arc<PythonValue>, marked: &mut Vec<bool>) {
        // 查找对象在 old_objects 中的索引
        if let Some(index) = self.old_objects.iter().position(|obj| Arc::ptr_eq(&obj.value, value)) {
            if !marked[index] {
                marked[index] = true;

                // 递归标记引用的对象
                match value.as_ref() {
                    PythonValue::List(list) => {
                        for item in list {
                            self.mark_object_young(item, &mut vec![false; self.young_objects.len()]);
                            self.mark_object_old(item, marked);
                        }
                    }
                    PythonValue::Tuple(tuple) => {
                        for item in tuple {
                            self.mark_object_young(item, &mut vec![false; self.young_objects.len()]);
                            self.mark_object_old(item, marked);
                        }
                    }
                    PythonValue::Dict(dict) => {
                        for (_, value) in dict {
                            self.mark_object_young(value, &mut vec![false; self.young_objects.len()]);
                            self.mark_object_old(value, marked);
                        }
                    }
                    PythonValue::Object(_, attrs) => {
                        for (_, value) in attrs {
                            self.mark_object_young(value, &mut vec![false; self.young_objects.len()]);
                            self.mark_object_old(value, marked);
                        }
                    }
                    _ => {
                        // 基本类型，不需要标记
                    }
                }
            }
        }
    }

    /// 清除年轻代未标记的对象
    fn sweep_young(&mut self, marked: &Vec<bool>) {
        let mut new_young_objects = Vec::new();
        let mut new_old_objects = Vec::new();

        for (i, object) in self.young_objects.iter().enumerate() {
            if marked[i] {
                // 如果对象年龄达到阈值，晋升到老年代
                if object.age >= 2 {
                    new_old_objects.push(GcObject { value: object.value.clone(), generation: Generation::Old, age: 0 });
                }
                else {
                    // 否则留在年轻代，年龄加 1
                    new_young_objects.push(GcObject {
                        value: object.value.clone(),
                        generation: Generation::Young,
                        age: object.age + 1,
                    });
                }
            }
        }

        self.young_objects = new_young_objects;
        self.old_objects.extend(new_old_objects);
    }

    /// 清除老年代未标记的对象
    fn sweep_old(&mut self, marked: &Vec<bool>) {
        let mut new_old_objects = Vec::new();

        for (i, object) in self.old_objects.iter().enumerate() {
            if marked[i] {
                new_old_objects.push(object.clone());
            }
        }

        self.old_objects = new_old_objects;
    }

    /// 停止垃圾回收器
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// 获取对象数量
    pub fn get_objects_count(&self) -> usize {
        self.young_objects.len() + self.old_objects.len()
    }

    /// 获取年轻代对象数量
    pub fn get_young_objects_count(&self) -> usize {
        self.young_objects.len()
    }

    /// 获取老年代对象数量
    pub fn get_old_objects_count(&self) -> usize {
        self.old_objects.len()
    }
}

/// 内存分配器
pub struct Allocator {
    /// 垃圾回收器
    gc: Arc<Mutex<GC>>,
}

impl Allocator {
    /// 创建新的内存分配器
    pub fn new(gc: Arc<Mutex<GC>>) -> Self {
        Self { gc }
    }

    /// 分配对象
    pub fn allocate(&self, value: PythonValue) -> Arc<PythonValue> {
        let arc_value = Arc::new(value);
        let mut gc = self.gc.lock().unwrap();
        gc.register_object(arc_value.clone());
        arc_value
    }

    /// 分配根对象
    pub fn allocate_root(&self, value: PythonValue) -> Arc<PythonValue> {
        let arc_value = Arc::new(value);
        let mut gc = self.gc.lock().unwrap();
        gc.register_root(arc_value.clone());
        arc_value
    }

    /// 触发垃圾回收
    pub fn collect(&self) {
        let mut gc = self.gc.lock().unwrap();
        gc.collect();
    }
}
