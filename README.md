# staticjson

staticjson is a code generation utility which is intended to generate json parsers for specific datatypes. The core target of staticjson are statically typed languages without reflection functionality. 

In statically typed languages with reflection we are able to see great generic json to object mapping frameworks. Without reflection the core principles of that frameworks can not be applied. The idea of staticjson is to bring similar usability to languages without reflection.

Currently staticjson is in its very early development state. It can not be used in real projects until now.

# Sample about staticjson

## staticjson code (file: interface.sjs)

```
 type KundeEntity(java-package="de.test.entities") {
   kundennummer:uint(
     primary_key="true", 
     mandatory="true"
   );
   name:string(mandatory="true", maxlen="50");
   vorname:string(mandatory="true", maxlen="50");
   strasse:string;
   postleitzahl:string;
   ort:string;
 }
 
 interface KundenRepository(
     java-package="de.test.interfaces",
     path="/repos") {
 
   // Method to retrieve a customer by its id
   getKundeById(id:int(path-param="id")) -> KundeEntity {
    method="GET",
    path="/kunde/{id}"
   }
   
   // Method to retrieve a list of all customers
   getKunden() -> KundeEntity[] {
     method="GET",
     path="/kunde"
   }
 }
```

## call the client code generation

```bash
 staticjson interface.sjs -t swift
```

## Usage of generated swift code

```swift
let kundeJSON = "{\"kundennummer\":1,\"name\":\"Mustermann\", \"vorname\":\"Max\", \"strasse\":\"Beispielstrasse\", \"postleitzahl\":\"12345\", \"ort\":\"Musterort\"}";
var kunde = KundeEntity.parse(kundeJSON);
kunde.name = "Mustermeier";
println("\nBeispiel: Kunde");
println(KundeEntity.serialize(kunde));
```
## call the server side code generation

```bash
 staticjson interface.sjs -t jaxrs
```

## Usage of generated java code

The code generation process generates java code for types and interfaces.
A type is a usual java bean with attributes, getters and setters. Additional
to getters and setters, it generates a validation function which supplies 
a useful way to check the validity of the content of the attributes
within the object.

```java
package de.test.entities;

import java.util.ArrayList;
import java.io.Serializable;

/**
* Generated Type for Entity KundeEntity 
*/
public class KundeEntity implements Serializable {

    private int kundennummer;   
    private String name;   
    private String vorname;   
    private String strasse;   
    private String postleitzahl;   
    private String ort;   

    public KundeEntity() {
        this.kundennummer = 0;
        this.name = null;
        this.vorname = null;
        this.strasse = null;
        this.postleitzahl = null;
        this.ort = null;
    }

    public int getKundennummer() {
        return this.kundennummer;
    }
    
    public void setKundennummer(int value) {
        this.kundennummer = value;
    }

    public String getName() {
        return this.name;
    }
    
    public void setName(String value) {
        this.name = value;
    }

    public String getVorname() {
        return this.vorname;
    }
    
    public void setVorname(String value) {
        this.vorname = value;
    }

    public String getStrasse() {
        return this.strasse;
    }
    
    public void setStrasse(String value) {
        this.strasse = value;
    }

    public String getPostleitzahl() {
        return this.postleitzahl;
    }
    
    public void setPostleitzahl(String value) {
        this.postleitzahl = value;
    }

    public String getOrt() {
        return this.ort;
    }
    
    public void setOrt(String value) {
        this.ort = value;
    }

    /**
    * The function isValid offert a validation function for the
    * mandatory attributes and other constraints of staticjson code
    * @param object to check
    * @return check result
    */
    public static boolean isValid(KundeEntity obj) {
        return obj != null
        && obj.kundennummer != 0
        && obj.name != null
        && (obj.name != null && 
            obj.name.length() <= 50)
        && obj.vorname != null
        && (obj.vorname != null && 
            obj.vorname.length() <= 50);
    }
}
```

Additional to the generated types, staticjson supports the generation of
interfaces. So its a fully featured interface definition language for http based
rest like interfaces. The following code listing is the result of the jaxrs generator
and shows a JAX-RS interface.

```java
package de.test.interfaces;

import java.util.ArrayList;
import javax.ws.rs.Path;
import javax.ws.rs.GET;
import de.test.entities.KundeEntity;

/**
* Generated Interface for KundenRepository with JAX-RS Annotations
*/
@Path("/repos")
public interface KundenRepository {

    /**
     * @param id 
     * @return KundeEntity
     */
    @GET
    @Path("/kunde/{id}")
    @Produces("application/json")
    public KundeEntity getKundeById(int id);

    /** 
     * @return ArrayList<KundeEntity>
     */
    @GET
    @Path("/kunde")
    @Produces("application/json")
    public ArrayList<KundeEntity> getKunden();
}
```