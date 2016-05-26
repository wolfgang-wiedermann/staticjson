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