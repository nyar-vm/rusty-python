console.log("Testing glob patterns...");

const test1 = import.meta.glob("../../../documentation/zh-hans/**/*.md", {
    query: "?raw",
    import: "default",
    eager: true,
});

console.log("Test 1 keys:", Object.keys(test1));

const test2 = import.meta.glob("../**/*.md", {
    query: "?raw",
    import: "default",
    eager: true,
});

console.log("Test 2 keys:", Object.keys(test2));
