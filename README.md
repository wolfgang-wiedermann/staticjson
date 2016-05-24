# staticjson

staticjson is my current proof of concept for a code generation utility which is intended to generate static code for json parsers for specific datatypes
and interfaces for HTTP based web APIs. The core target of staticjson is to enable easy use of web APIs to statically typed languages without reflection functionality. 

In statically typed languages with reflection we are able to see great generic json to object mapping frameworks. Without reflection the core principles of that frameworks can not be applied. The idea of staticjson is to bring similar usability to languages without reflection.

Currently staticjson is in its very early development state. It can not be used in real projects until now but it already shows the potential power 
of using annotated IDLs in the context of REST APIs.

# Current state of development 

The current version of staticjson is able to produce working serverside Java code (serverside stub) annotated with JAX-RS annotations and 
JavaScript proxys based on jQuerys ajax functionality. Java and JavaScript both are no statically typed languages without reflection functionality,
but even with them the power of this IDL based approach is already visible.

# How staticjson works

The following sample shows how staticjson works. For this it step by step shows how to generate a serverside Java stub and
its corresponding JavaScript client proxy.

## staticjson code (file: sample.sjs)

```
type Customer (
        java-package="de.ww.sample.entities",
        jpa-entity="true",
        jpa-table="tbl_customer",
        js-namespace="de.ww.sample.entities"
    ) {
  
  customer_id:int(
    jpa-id="true", 
    jpa-generate-value="true"
  );

  prename:string(maxlen="50");
  surname:string(maxlen="50");
}

interface CustomerRepository (
        java-package="de.ww.sample",
        js-namespace="de.ww.sample.proxy",
        path="/api"
    ) {

  // Get a Customer by its id
  getCustomerById(id:int(path-param="id")) -> Customer {
    method="GET",
    path="/customer/{id}"
  }

  // Find a customer by its pre- or surname
  findCustomer(prename:string(query-param="prename"),
               surname:string(query-param="surname")) -> customer[] {
    method="GET",
    path="/customer"
  }

  // Create a new customer and return it with its server generated id
  createCustomer(c:Customer) -> Customer {
    method="POST",
    path="/customer"
  }

  // Deletes a customer by its id and returns the id at success
  deleteCustomer(id:int(path-param="id")) -> int {
    method="DELETE",
    path="/customer/{id}"
  }
}

```

## Call the server side code generation

```bash
staticjson -t jaxrs -o java/ src/sample.sjs
```

## Usage of generated java code

The code generation process generates java code for types and interfaces.
A type is a usual java bean with attributes, getters and setters. Additional
to getters and setters, it generates a validation function which supplies 
a useful way to check the validity of the content of the attributes
within the object.

```java
/*
* de/ww/sample/CustomerRepository.java
*/
package de.ww.sample;

import java.util.ArrayList;
import javax.ws.rs.Path;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.DELETE;
import javax.ws.rs.Produces;
import javax.ws.rs.QueryParam;
import javax.ws.rs.PathParam;
import javax.ws.rs.Consumes;
import de.ww.sample.entities.Customer;

/**
* Generated Interface for CustomerRepository with JAX-RS Annotations
*/
@Path("/api")
public interface CustomerRepository {

    /**
     * @param id 
     * @return Customer
     */
    @GET
    @Path("/customer/{id}")
    @Produces("application/json")
    public Customer getCustomerById(@PathParam("id") int id);

    /**
     * @param prename
     * @param surname 
     * @return ArrayList<customer>
     */
    @GET
    @Path("/customer")
    @Produces("application/json")
    public ArrayList<customer> findCustomer(@QueryParam("prename") String prename, @QueryParam("surname") String surname);

    /**
     * @param c 
     * @return Customer
     */
    @POST
    @Path("/customer")
    @Produces("application/json")
    @Consumes("application/json")
    public Customer createCustomer(Customer c);

    /**
     * @param id 
     * @return int
     */
    @DELETE
    @Path("/customer/{id}")
    public int deleteCustomer(@PathParam("id") int id);
}
```

Additional to the generated types, staticjson supports the generation of
interfaces. So its a fully featured interface definition language for http based
rest like interfaces. The following code listing is the result of the jaxrs generator
and shows a JAX-RS interface.

```java

/*
* de/ww/sample/entities/Customer.java
*/
package de.ww.sample.entities;

import java.util.ArrayList;
import java.io.Serializable;
import javax.persistence.Entity;
import javax.persistence.Table;
import javax.persistence.Id;

/**
* Generated Type for Entity Customer 
*/
@Entity
@Table(name="tbl_customer")
public class Customer implements Serializable {

  private static final long serialVersionUID = 1L;

    private int customerId;   
    private String prename;   
    private String surname;   

    public Customer() {
        this.customerId = 0;
        this.prename = null;
        this.surname = null;
    }

    @Id
    public int getCustomerId() {
        return this.customerId;
    }
    
    public void setCustomerId(int value) {
        this.customerId = value;
    }

    public String getPrename() {
        return this.prename;
    }
    
    public void setPrename(String value) {
        this.prename = value;
    }

    public String getSurname() {
        return this.surname;
    }
    
    public void setSurname(String value) {
        this.surname = value;
    }

    /**
    * The function isValid offert a validation function for the
    * mandatory attributes and other constraints of staticjson code
    * @param object to check
    * @return check result
    */
    public static boolean isValid(Customer obj) {
        return obj != null
        && (obj.prename != null && 
            obj.prename.length() <= 50)
        && (obj.surname != null && 
            obj.surname.length() <= 50);
    }
}

```

## Call the client proxy code generation

```bash
staticjson -t jquery -o js/ src/sample.sjs
```

## Generated JavaScript

The code generation process generates the Javascript proxy code which looks like the following
code sample.

```javascript
// Namespace generieren
var de = de || {};
de.ww = de.ww || {};
de.ww.sample = de.ww.sample || {};
de.ww.sample.proxy = de.ww.sample.proxy || {};


/**
* Generated Proxy for CustomerRepository
*/
de.ww.sample.proxy.CustomerRepositoryProxy = function(urlBase) {
    var self = this;

    // URL-Basis aufbauen
    self.url = urlBase;
    self.url += "/api";  
    

    /**
     * @param id 
     * @return Customer
     */ 
    self.getCustomerById = function(id, successHandler, errorHandler) { 
        // HTTP-GET call    
        var method = "GET";
        var queryParams = ""; 
        var path = self.url + "/customer/{id}";
        
	    path = path.replace("{id}", encodeURIComponent(id)); 
        if(queryParams.length > 0) {
            path = path + "?" + queryParams;
        }        
        // DEBUG OUTPUT:
        console.log(method + " " + path);
        
        $.ajax({
            "url": path,
            "method": method,
            "dataType": "json",
            "success": successHandler,
            "error": errorHandler
        });
    }

    /**
     * @param prename
     * @param surname 
     * @return customer
     */ 
    self.findCustomer = function(prename, surname, successHandler, errorHandler) { 
        // HTTP-GET call    
        var method = "GET";
        var queryParams = ""; 
        var path = self.url + "/customer";
// Namespace generieren
var de = de || {};
de.ww = de.ww || {};
de.ww.sample = de.ww.sample || {};
de.ww.sample.proxy = de.ww.sample.proxy || {};


/**
* Generated Proxy for CustomerRepository
*/
de.ww.sample.proxy.CustomerRepositoryProxy = function(urlBase) {
    var self = this;

    // URL-Basis aufbauen
    self.url = urlBase;
    self.url += "/api";  
    

    /**
     * @param id 
     * @return Customer
     */ 
    self.getCustomerById = function(id, successHandler, errorHandler) { 
        // HTTP-GET call    
        var method = "GET";
        var queryParams = ""; 
        var path = self.url + "/customer/{id}";
        
	    path = path.replace("{id}", encodeURIComponent(id)); 
        if(queryParams.length > 0) {
            path = path + "?" + queryParams;
        }        
        // DEBUG OUTPUT:
        console.log(method + " " + path);
        
        $.ajax({
            "url": path,
            "method": method,
            "dataType": "json",
            "success": successHandler,
            "error": errorHandler
        });
    }

    /**
     * @param prename
     * @param surname 
     * @return customer
     */ 
    self.findCustomer = function(prename, surname, successHandler, errorHandler) { 
        // HTTP-GET call    
        var method = "GET";
        var queryParams = ""; 
        var path = self.url + "/customer";

        if(queryParams.length > 0) {
            queryParams += "&";
        }                
        queryParams += "prename=" + encodeURIComponent(prename);            
        if(queryParams.length > 0) {
            queryParams += "&";
        }                
        queryParams += "surname=" + encodeURIComponent(surname);             
        if(queryParams.length > 0) {
            path = path + "?" + queryParams;
        }        
        // DEBUG OUTPUT:
        console.log(method + " " + path);
        
        $.ajax({
            "url": path,
            "method": method,
            "dataType": "json",
            "success": successHandler,
            "error": errorHandler
        });
    }

    /**
     * @param c 
     * @return Customer
     */ 
    self.createCustomer = function(c, successHandler, errorHandler) { 
        // HTTP-POST call  
        var method = "POST";
        var queryParams = ""; 
        var path = self.url + "/customer";
 
        if(queryParams.length > 0) {
            path = path + "?" + queryParams;
        }        
        // DEBUG OUTPUT:
        console.log(method + " " + path);
        
        $.ajax({
            "url": path,
            "method": method,
            "contentType":'application/json; charset=UTF-8',
            "data": JSON.stringify(c), 
            "dataType": "json",
            "success": successHandler,
            "error": errorHandler
        });
    }

    /**
     * @param id 
     * @return int
     */ 
    self.deleteCustomer = function(id, successHandler, errorHandler) { 
        // HTTP-DELETE call    	
        var method = "DELETE";
        var queryParams = ""; 
        var path = self.url + "/customer/{id}";
        
	    path = path.replace("{id}", encodeURIComponent(id)); 
        if(queryParams.length > 0) {
            path = path + "?" + queryParams;
        }        
        // DEBUG OUTPUT:
        console.log(method + " " + path);
        
        $.ajax({
            "url": path,
            "method": method,
            "dataType": "json",
            "success": successHandler,
            "error": errorHandler
        });
    }
}
        if(queryParams.length > 0) {
            queryParams += "&";
        }                
        queryParams += "prename=" + encodeURIComponent(prename);            
        if(queryParams.length > 0) {
            queryParams += "&";
        }                
        queryParams += "surname=" + encodeURIComponent(surname);             
        if(queryParams.length > 0) {
            path = path + "?" + queryParams;
        }        
        // DEBUG OUTPUT:
        console.log(method + " " + path);
        
        $.ajax({
            "url": path,
            "method": method,
            "dataType": "json",
            "success": successHandler,
            "error": errorHandler
        });
    }

    /**
     * @param c 
     * @return Customer
     */ 
    self.createCustomer = function(c, successHandler, errorHandler) { 
        // HTTP-POST call  
        var method = "POST";
        var queryParams = ""; 
        var path = self.url + "/customer";
 
        if(queryParams.length > 0) {
            path = path + "?" + queryParams;
        }        
        // DEBUG OUTPUT:
        console.log(method + " " + path);
        
        $.ajax({
            "url": path,
            "method": method,
            "contentType":'application/json; charset=UTF-8',
            "data": JSON.stringify(c), 
            "dataType": "json",
            "success": successHandler,
            "error": errorHandler
        });
    }

    /**
     * @param id 
     * @return int
     */ 
    self.deleteCustomer = function(id, successHandler, errorHandler) { 
        // HTTP-DELETE call    	
        var method = "DELETE";
        var queryParams = ""; 
        var path = self.url + "/customer/{id}";
        
	    path = path.replace("{id}", encodeURIComponent(id)); 
        if(queryParams.length > 0) {
            path = path + "?" + queryParams;
        }        
        // DEBUG OUTPUT:
        console.log(method + " " + path);
        
        $.ajax({
            "url": path,
            "method": method,
            "dataType": "json",
            "success": successHandler,
            "error": errorHandler
        });
    }
}
```