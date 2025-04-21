# {{ service.name }} API Documentation

## Methods

{% for method in service.methods %}
### {{ method.name }}

#### Request
| Field Name | Type |
|------------|------|
{% for field in method.request.fields %}
| {{ field.name }} | {{ field.type }} |
{% endfor %}

#### Response
| Field Name | Type |
|------------|------|
{% for field in method.response.fields %}
| {{ field.name }} | {{ field.type }} |
{% endfor %}
{% endfor %}