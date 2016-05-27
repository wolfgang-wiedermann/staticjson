# staticjson

Das Projekt staticjson ist die Basis für meine aktuelle Untersuchung zur Nützlichkeit von Annotierten IDLs zur Verbesserung des Entwicklungsprozesses von webbasierten Programmierschnittstellen im REST- und RPC-Stil. 

# Entwicklungsgeschichte

Auslöser für meine Untersuchung war, dass ich im Frühling 2015 im Rahmen meiner ersten Versuche mit der damals von Apple neu
vorgestellten Programmiersprache Swift feststellen musste, dass aufgrund verschiedener Merkmale der Sprache das Entwickeln einfacher Client-Proxies wesentlich komplizierter war,
als ich das von den Sprachen Java, C# und JavaScript gewohnt war. Für diesen Unterschied ist vor allem das fehlen umfangreicher Reflection Features bei gleichzeitiger Nutzung
eines statischen Typsystems verantwortlich.

In statisch typisierten Sprachen mit umfassender Reflection (wie Java oder C#) werden derzeit hervorragende generische Frameworks zur Serialisierung und Deserialisierung von Objekten nach JSON angeboten. In Sprachen mit statischer Typisierung, die keine Reflection unterstützen können die hierzu verwendeten Konzepte nicht angewendet werden. Deshalb war die initiale Idee hinter staticjson eine Lösung zu schaffen, die ähnlichen Komfort bei der Verwendung von JSON auch in die Programmierung mit solchen Sprachen bringt. Hierzu wurde anstelle des generischen Ansatzes ein generierender Ansatz auf der Basis einer IDL gewählt, die mindestens genau jene Informationen enthält auf denen die korrespondierenden generischen Ansätze aus
den Sprachen Java und C# basieren.

Im Laufe der weiteren Untersuchung sind dann zusätzliche interessante Aspekte rund um die Generierung sowohl von clientseitigem Proxy-Code als auch von serverseitigen Stubs zum
Vorschein gekommen, die dazu führten, dass die Generierung von JSON-Parsern vorerst zugunsten der Generierung von Proxys und Stubs in Java und C# zurückgestellt wurde. 

Einer dieser Aspekte ist, dass in derzeitigen webbasierten Anwendungen üblicherweise der client- und serverseitige Code in zwei verschiedenen Programmiersprachen verfasst wird. Dabei wird auf beiden Seiten der Schnittstelle viel redundanter Code verfasst. Außerdem besteht eine gewisse Herausvorderung darin, die beiden Seiten der Implementierung der Schnittstelle synchron zu halten.

Ein anderer Aspekt ist, dass webbasierte Schnittstellen in Zeiten mobiler Anwendungen neben der Nutzung aus ebenfalls webbasierten Oberflächen heraus häufig auch von verschiedenen Apps von mobilen Endgeräten aus genutzt werden sollen. Optimale Performance versprechen hier die nativen Implementierungen für die jeweilige mobile Betriebssystemplattform. Das bedeutet, dass die Schnittstellen Clientseitig mindestens aus Swift/Objective-C für iOS, Java für Android und C# für mobile Windowsgeräte verwendet werden können müssen. Hier ist zu vermuten, dass ein generierender Ansatz auf der Basis einer annotierten IDL hilfreich ist, eine über die drei Plattformen hinweg konsistente Nutzung der Schnittstelle zu ermöglichen und Fehler bei Schnittstellenänderungen zu vermeiden. 

Der nächste Schritt im Rahmen der Untersuchung ist der Einsatz als Generator für serverseitigen C#- und clientseitigen JavaScript-Code im Rahmen eines realen 
Entwicklungsprojekts an der KDV-FH.

# Aktueller Entwicklungsstand 

Im aktuellen Entwicklungsstand produziert staticjson funktionsfähigen serverseitigen Java-Code (serverside stub) in Form von vollständig annotierten JPA Entites und mit JAX-RS Annotationen annotierten Interfaces und funktionsfähige JavaScript-Proxys basierend auf der jQuery-Ajax-Funktion. 

Da weder Java noch JavaScript statisch typisierte Sprachen ohne Reflection sind wurde hier die generierung von typisierten JSON-Parsern und Serialisierern zugunsten der gut funktionierenden JSON Serialisierungsmechnismen der beiden Sprachen eingespart. 

Der vermutete Nutzwert bezüglich der konsistenten Verwendung der Schnittstelle im Client und im Server sowie die Einsparung von in mehreren Sprachen zu programmierenden identischen Artefakten tritt hier in gleichem Umfang auf und kann so zur Untersuchung der Auswirkungen der Verwendung einer annotierten IDL in der Entwicklung von auf REST-Schnittstellen basierenden Anwendungssystemen herangezogen werden.

# Wie funktioniert staticjson?

Das nachfolgend dragestellte Beispiel zeigt, wie staticjson funktioniert. Dazu wird Schritt für Schritt die Generierung eines serverseitigen Java-Stubs und eines JavaScript-Proxys für den Client dargestellt.

## staticjson Code (file: sample.sjs)

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

## Aufruf der Code-Generierung für den Java-Code

```bash
staticjson -t jaxrs -o java/ src/sample.sjs
```

## Erläuterung des Generierungsergebnisses

Der Generierungsprozess hat aus dem obigen staticjson-Code Java-Code für den Typen "Customer" und
das Interface "CustomerRepository" generiert. 

Das erste Code-Listing zeigt den generierten Code für den Typen "Customer". Er wird als serialisierbare
öffentliche Java-Klasse mit privaten Attributen und für den Attributzugriff bestimmten gettern und settern
generiert. Ens handelt sich also um eine klassische Java-Bean. Wenn im staticjson-Code entsprechend angegeben
kann diese einfach um Annotationen der Java Persistence API (JPA) ergänzt werden.

Neben den gettern und settern wird auch noch eine Methode generiert, mit der die Gültigkeit der 
Attributwerte geprüft werden kann.

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

Neben den Typen können in staticjson auch Interfaces defniert werden. Sie unterstützt damit die
vollständige Spezifikation der Schnittstelle, bestehend aus deren angebotenem Funktionsumfang 
einschließlich der in dem Methodensignaturen verwendeten Typen.

Das nachfolgende Code-Listing zeigt den generierten Code für das Interface "CustomerRepository"
einschließlich der zugehörigen JAX-RS Annotationen. 

Die Möglichkeit zur Annotation im Interface bei JAX-RS Schnittstellen erlaubt eine saubere
Trennung zwischen dem generierten und dem manuell zu programmierenden Code, sodass bei einer
späteren Neugenerierung des Interfaces kein manuell erstellert Code verloren geht. 

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

## Aufruf der Code-Generierung für den JavaScript-Code

```bash
staticjson -t jquery -o js/ src/sample.sjs
```

## Generierter JavaScript-Code

Der Generierungsprozess erzeugt den folgenden Javascript-Proxy-Code.

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

# Verwendung des generierten JavaScript-Proxys

Das folgende Code-Listing zeigt am Beispiel des Aufrufs der deleteCustomer-Methode, wie
der generierte Proxy verwendet werden kann.

```javascript

// Neue Proxy-Instanz unter Angabe der Basis-URL erzeugen
var proxy = new de.ww.sample.proxy.CustomerRepositoryProxy("https://irgendwo.com/application");

//
// Eine Funktion aufrufen
// Verbindet mit irgendwo.com an Port 443 per HTTPS und sendet folgenden HTTP-Aufruf
// DELETE /application/customer/12
//
proxy.deleteCustomer(12,
  function(success) {
    console.log("Der Kunde wurde erfolgreich gelöscht");
  },
  function(error) {
    console.log("Fehler beim Löschen des Kunden aufgetreten");
  }
);

```