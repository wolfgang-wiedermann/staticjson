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