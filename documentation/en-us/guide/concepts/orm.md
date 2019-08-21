# ORM Concept

## What is ORM

ORM (Object-Relational Mapping) is a programming technique used in object-oriented programming languages to establish a mapping relationship between data in relational databases and object models. It allows developers to manipulate databases in an object-oriented way without directly writing SQL statements.

## Advantages of ORM

1. **Simplified Development**: Developers can use familiar object-oriented programming paradigms without learning SQL syntax.
2. **Improved Maintainability**: Through object models, code is clearer, easier to understand, and maintain.
3. **Reduced Errors**: Avoid syntax errors and security issues (such as SQL injection) caused by manually writing SQL statements.
4. **Database Independence**: Through ORM abstraction, applications can switch between different database systems without modifying business logic code.

## ORM Implementation in RBQ

RBQ has a built-in self-developed ORM system with the following features:

1. **Declarative Modeling**: Use `.rbq` files to define data models with concise and clear syntax.
2. **Compile-time Code Generation**: Generate optimal Rust code at compile time, no runtime reflection, excellent performance.
3. **Type Safety**: All database operations are type-safe, avoiding runtime errors.
4. **Query Builder**: Provide type-safe query builder with chainable calls.
5. **Transaction Support**: Built-in transaction management to ensure data consistency.
6. **Relationship Mapping**: Support one-to-one, one-to-many, many-to-many relationship mapping.

## Example

```rbq
model User {
    id: i64 = 0;
    @unique username: string;
    email: string;
    created_at: datetime = now();
    
    // Relationship
    posts: Post[];
}

model Post {
    id: i64 = 0;
    title: string;
    content: string;
    author_id: i64;
    
    // Foreign key association
    @belongs_to author: User;
    created_at: datetime = now();
}
```

The generated Rust code can be used like this:

```rust
// Database operation
let db = user::db::connect().await.unwrap();

// Create user
let user = User {
    id: 0,
    username: "john",
    email: "john@example.com",
    created_at: chrono::Utc::now(),
    posts: None,
};
let saved_user = user.insert(&db).await.unwrap();

// Query user
let found_user = User::find_by_id(&db, saved_user.id).await.unwrap();

// Use query builder
let users = User::query()
    .filter(|u| u.username.like("%john%"))
    .order_by(|u| u.created_at.desc())
    .fetch(&db).await.unwrap();
```