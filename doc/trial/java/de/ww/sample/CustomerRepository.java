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