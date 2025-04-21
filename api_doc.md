# API 文档

## 命名空间

- **volo.example**

---

## 结构体

### Item

| 字段名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| id | Type(I64, Annotations([])) | 是 | |
| title | Type(String, Annotations([])) | 是 | |
| content | Type(String, Annotations([])) | 是 | |
| extra | Type(Map { key: Type(String, Annotations([])), value: Type(String, Annotations([])), cpp_type: None }, Annotations([])) | 否 | |

---

### GetItemRequest

| 字段名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| id | Type(I64, Annotations([])) | 是 | |

---

### GetItemResponse

| 字段名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| item | Type(Path(Path { segments: [Ident("Item")] }), Annotations([])) | 是 | |

---

## 服务

### ItemService

#### 方法

##### GetItem

- **请求参数**：Type(Path(Path { segments: [Ident("GetItemRequest")] }), Annotations([]))

- **返回结果**：Type(Path(Path { segments: [Ident("GetItemResponse")] }), Annotations([]))

---

