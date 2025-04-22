# API 文档

## 命名空间

- **{{ package }}**

## 结构体

{% for s in structs -%}
### {{ s.name }}

| 字段名 | 类型                | 必填 | 说明 |
|--------|---------------------|------|------|
{% for f in s.fields -%}
| {{ f.name }} | {{ f["type"] }} | {% if f.attribute | contains(substring="Required") %}是{% else %}否{% endif %} | |
{% endfor -%}

---
{% endfor -%}

## 服务

{% for service in services -%}
### {{ service.name }}

#### 方法
{% for method in service.methods -%}
##### {{ method.name }}

- **请求参数：** {{ method.request.name }}
{% for f in method.request.fields -%}
- {{ f.name }}: {{ f["type"] }} ({% if f.attribute | contains(substring="Required") %}是{% else %}否{% endif %})
{% endfor -%}

- **返回结果：** {{ method.response.name }}
{% for f in method.response.fields -%}
- {{ f.name }}: {{ f["type"] }} ({% if f.attribute | contains(substring="Required") %}是{% else %}否{% endif %})
{% endfor -%}

{% endfor -%}
{% endfor -%}