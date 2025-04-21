# API 文档

## 命名空间

- **volo.example**

---

## 结构体

### Item

| 字段名  | 类型                | 必填 | 说明 |
|---------|---------------------|------|------|
| id      | i64                 | 是   |      |
| title   | string              | 是   |      |
| content | string              | 是   |      |
| extra   | map<string, string> | 否   |      |

---

### GetItemRequest

| 字段名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| id     | i64  | 是   |      |

---

### GetItemResponse

| 字段名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| item   | Item | 是   |      |

---

## 服务

### ItemService

#### 方法

##### GetItem

- **请求参数**：GetItemRequest
    - id: i64

- **返回结果**：GetItemResponse
    - item: Item

---