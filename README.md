# 📦 Contract Templates (Soroban Smart Contract)

<img width="1920" height="1080" alt="Screenshot (1)" src="https://github.com/user-attachments/assets/abf5eb2f-d3df-4c6c-8ba2-065f965022d8" />

## 📖 Project Description

Contract Templates is a smart contract built using the Soroban SDK on the Stellar blockchain. It enables users to store and retrieve reusable contract templates in a decentralized and immutable way.

This project is designed to simplify how developers and users manage commonly used contract formats (e.g., NDAs, agreements, DAO rules) directly on-chain.

---

## ⚙️ What it does

The contract provides a simple on-chain storage system where:

* Users can create and store contract templates using a unique name
* Users can retrieve templates anytime using that name
* Templates are stored persistently on the Stellar blockchain

It acts as a decentralized registry for reusable contract structures.

---

## ✨ Features

* 📌 Store contract templates on-chain
* 🔍 Retrieve templates by name
* 🧱 Uses Soroban smart contract framework
* ⚡ Lightweight and efficient design
* 🌐 Fully decentralized storage
* 🔁 Update existing templates

---

## 📜 Smart Contract

### 🔗 Deployed Contract Address

```
CBHCKSVQXJ2LFN7OJEJXNINSFNR63IKYRKDTBHK64URCLPYAEOL6555K
```

You can view the contract on Stellar explorers like:

* https://stellar.expert
* https://soroban.stellar.org

---

## 🛠️ Tech Stack

* Rust
* Soroban SDK
* Stellar Blockchain

---

## 📂 Contract Functions

### `set_template(name, content)`

Stores or updates a contract template.

* **name**: Unique identifier (Symbol)
* **content**: Template text (String)

---

### `get_template(name)`

Retrieves a stored template.

* **Returns**: Template content if it exists

---

## 🧪 Example Usage

### Store Template

```
set_template("nda", "Non-disclosure agreement template")
```

### Retrieve Template

```
get_template("nda")
```

---

## 📌 Future Improvements

* 🔐 Add access control (ownership)
* 🧾 Template versioning
* 🏷 Categories/tags for templates
* 🌍 Public/private template visibility
* 🎨 Frontend UI for easier interaction

---

## 📜 License

MIT License

---

## 🙌 Acknowledgements

Built using Soroban smart contracts on Stellar.
