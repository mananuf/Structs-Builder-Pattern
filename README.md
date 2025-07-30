# 🦀 Rust Builder Pattern Example

This project demonstrates the **Builder Pattern** in Rust using a simple **User Account** example. The **Builder Pattern** is a creational design pattern that allows step-by-step construction of complex objects, making your code cleaner, safer, and more flexible.

---

## 📖 **What is the Builder Pattern?**

The **Builder Pattern** is used to **create objects with many optional fields** without having to write multiple constructors.

Instead of creating a huge constructor with multiple optional parameters (which becomes unreadable), we use a **builder struct** to:

1. Initialize required fields in the `new` method.
2. Configure optional fields using chainable methods.
3. Finalize object creation using the `build()` method.

---

## 💡 **Significance in Rust**

1. **Immutability & Safety**

    * Helps avoid partially initialized objects by separating the construction process.
2. **Readability & Flexibility**

    * Makes object creation expressive and chainable.
3. **Optional Fields Handling**

    * Easily support optional or default fields using `Option<T>` and `unwrap_or_default()`.
4. **Prevents Long Parameter Lists**

    * Instead of passing many `None` or default values in constructors, you only specify what you need.

---

## 🏦 **Use Case Example**

Here’s a **User Account** example for a banking application:

* **Required Fields:** firstname, lastname, account\_number
* **Optional Fields:** account\_type, age, address

With the **Builder Pattern**, we can create:

* A user with **only required fields**.
* A user with **some or all optional fields**, without repetitive boilerplate code.

---

## 🛠 **Code Structure**

### 1. **Enums & Structs**

```rust
#[derive(Debug, Clone)]
enum AccountType {
    Savings,
    Current
}

#[derive(Debug, Clone)]
struct Address {
    country: String,
    state: String,
    city: String
}

#[derive(Debug)]
struct User {
    fisrtname: String,
    lastname: String,
    account_number: String,
    account_type: AccountType,
    age: u8,
    address: Address
}
```

* **`User`**: Main struct we want to build.
* **`Address`**: Optional nested struct.
* **`AccountType`**: Enum to demonstrate type-safe account types.

---

### 2. **UserBuilder Struct**

```rust
#[derive(Default, Debug)]
struct UserBuilder {
    fisrtname: String,
    lastname: String,
    account_number: String,
    account_type: Option<AccountType>,
    age: Option<u8>,
    address: Option<Address>
}
```

* Holds **all required and optional fields**.
* Optional fields are wrapped in `Option<T>`.

---

### 3. **Builder Methods**

```rust
impl UserBuilder {
    fn account_type(&mut self, account_type: AccountType) -> &mut Self { ... }
    fn age(&mut self, age: u8) -> &mut Self { ... }
    fn address(&mut self, address: Address) -> &mut Self { ... }
    fn build(&mut self) -> User { ... }
}
```

* Methods return `&mut Self` to enable **method chaining**:

```rust
let user = User::new("John".into(), "Doe".into(), "12345".into())
    .age(30)
    .account_type(AccountType::Current)
    .build();
```

---

## 🔹 **Example Output**

**Manji (with defaults)**:

```rust
User {
    fisrtname: "Manji",
    lastname: "Gar",
    account_number: "12345678",
    account_type: Savings,
    age: 0,
    address: Address { country: "", state: "", city: "" }
}
```

**James (with custom fields)**:

```rust
User {
    fisrtname: "James",
    lastname: "Victor",
    account_number: "12345678",
    account_type: Current,
    age: 12,
    address: Address { country: "Cotonou", state: "", city: "" }
}
```

---

## ⚡ **Use Cases in Real Projects**

1. **Configuration Builders**

    * Building complex configurations for servers, APIs, or SDK clients.
2. **Database Models with Optional Fields**

    * Insert records with optional metadata without multiple constructors.
3. **UI Component Builders (WebAssembly / GUI)**

    * Fluent syntax for component creation with optional styling or behaviors.
4. **Game Development**

    * Create characters or items with optional attributes.

---

## 🏁 **Benefits Recap**

* Clean and readable object creation
* Supports optional fields gracefully
* Encourages method chaining (fluent API)
* Prevents invalid/partial object states