{{ foreach type in types }}
{{ new-file }}
<html>
<head><title>Documentation for Type: {{ type.name }}</title></head>
<body>
<h1>Documentation for Type: {{ type.name }}</h1>
{{ foreach attribute in type.attributes }} 
  <h2>Attribut: {{ attribute.name }} -> {{ attribute.type }}</h2>
  {{ foreach param in attribute.params }}
  <p>
    <b>{{ param.name }} : </b><span> {{ param.value }}</span><br/>
  </p>
  {{ /foreach }}
{{ /foreach }}
</body>
</html>
{{ /foreach }}
