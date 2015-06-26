# staticjson

staticjson is a code generation utility which is intended to generate json parsers for specific datatypes. The core target of staticjson are statically typed languages without reflection functionality. 

In statically typed languages with reflection we are able to see great generic json to object mapping frameworks. Without reflection the core principles of that frameworks can not be applied. The idea of staticjson is to bring similar usability to languages without reflection.

Currently staticjson is in its very early development state. It can not be used in real projects until now.

# Sample about staticjson

## staticjson code (file: model.sjs)

```
 type KundeEntity {
   kundennummer:uint(primary_key="true");
   name:string;
   vorname:string;
   strasse:string;
   postleitzahl:string;
   ort:string;
 }
```

## call the code generation

```bash
 staticjson model.sjs -t swift
```

## Usage of generated swift code

```swift
let kundeJSON = "{\"kundennummer\":1,\"name\":\"Mustermann\", \"vorname\":\"Max\", \"strasse\":\"Beispielstrasse\", \"postleitzahl\":\"12345\", \"ort\":\"Musterort\"}";
var kunde = KundeEntity.parse(kundeJSON);
kunde.name = "Mustermeier";
println("\nBeispiel: Kunde");
println(KundeEntity.serialize(kunde));
``
