# Rust Smart Contract a non-profit and it's Inventory and Orders and their status
###################################################################################

  This is for Hackathon submition

  In the long-run this contract will be accessed by two different front
  end websites. 
  
1.  The application portal for a non-profit supplier that in this example is a manufacturor of 
Bags and cases of bags of product. 
    
2.  The web site for a non profit that sells the product in this example on their website and pre-purchases inventory that is held by the manufacturer and drop shipped to customers that are making purchases as a donation those orders are added to the contract.

Example Description
###################

  1.  The non-profit first applies for a non-profit account with the manufacturer via the  manufacturers website.  The contract is deployed with the non-profit contract with a status of "Applicant".
  
  2.  The manufacturer then approves the non-profit organizaiton.  The Status is then changed to "Approved".
  
  3.  The non-profit then pre-orders inventory of bags and cases of product that the manufacturer   can hold and drop-ship as orders are made from the non-profit website.

  4.  Orders are then placed on the non-profit website and paid for either in NEAR or by credit card" payment. This will be implemented in a later version of this contract.
  
  5.  The order is then added to the list of orders for the non-profit.  The status of the order at that time will be "ordered".
  
  6.  The manufacturer then drop-ships the order to the address on the order that was added to the contract.
  
  7.  The status of the order is then updated to "Shipped".
  
  8.  Additional status changes for tracking delivery of the product can also be added to the order record.
  
  9.  In future versions of this contract I invision adding a "Factory" contract and adding a collection of    wholesale dostributors to this contract, or creating multiple instances of this contract that exist only as long as the wholesale customer/distributor is active.

  Also in the future we will probably need to ad some mechinism to store the end customer or person donating's invormation so that it is not visible on the blockchain.  maybe create subaccounts for each customer with address information in those accounts.  Anyway this needs to be addressed
  
  10. The unit test for this contract creates the contract puts the non-profit in "Applicant" status, updates them to "Approved" status then adds an order to the order list.  Also all get and set methods are tested in addition to an encription method for storing account information encripted.  Then verifying that the account Id matches the encripted value stored.

FINALLY

The following shell scripts are provided and can be used either with bash or sh.
The following environment variables are required for the following shell scripts.

Environment Variables that need to be set before using scripts:
###############################################################

Export CONTRACTS = Contract name assigned by deployment

Export OWNER = Account controlled by you

Scripts Provided
################
1.  dev-deploy.sh  -- cleans up, builds release version of contract and deploy's contract on near environment
2.  test.sh -- builds contract and runs the Unit test modules
3.  build.sh -- builds the project and moves the wasm file into the res directory

CONTRACT Public functions
#########################

# Initialization of Contract function
    pub fn new() -> Self
         
# Add a customer order to the HashMap list of Orders for this non-profit    
    pub fn set_customer_order(&mut self,
                        ec_account: AccountId,
                        ec_customer_id: u8,
                        ec_order_id: u64,
                        ec_bags_ordered: u8,
                        ec_cases_ordered: u8,
                        ec_bag_purchase_price: f64,
                        ec_cases_purchase_price: f64,
                        ec_shipping_address1: String,
                        ec_shipping_address2: String,
                        ec_shipping_address3: String,
                        ec_apt_or_unit: String,
                        ec_state: String,
                        ec_zip: String,
                        ec_shipping_cost: f64,
                        ec_order_status: String)
  
# get funtion for the HashMap Order List
    pub fn get_customer_orders(&self) -> &HashMap<u64, EndCustomerOrder>

# set function for the account mane
    pub fn set_entity_account(&mut self, account: String) 

# get function for the account name
    pub fn get_entity_account(&mut self) -> &String 

# set function for the main account Id
    pub fn set_entity_main_account(&mut self, account: AccountId) 

# get function for the account name
    pub fn get_entity_main_account(&mut self) -> &AccountId 

# set function for the Wholesale distributor entity name
    pub fn set_entity_name(&mut self, name: String) 
    
# get function for the Wholesale distributor entity name
    pub fn get_entity_name(&mut self) -> &String 

# set function for wholesale distributor status
    pub fn set_entity_status(&mut self, status: String) 

# get function for the wholesale distributor entity status
    pub fn get_entity_status(&mut self) -> &String 
    
# set function for entity description
    pub fn set_entity_description(&mut self, description: String) 

# get function for the entity description
    pub fn get_entity_description(&mut self) -> &String 
    
# set function for the allowed number of bags to have in inventory
    pub fn set_entity_allowed_inventory_bags(&mut self, allowed_inventory_bags: u8) 

# get function for the allowed number of bags ti gave in inventory
    pub fn get_entity_allowed_inventory_bags(&mut self) -> &u8 
    
# set function for current bag inventory for this wholesale distributor
    pub fn set_entity_bag_inventory(&mut self, bag_inventory: u8)

# get function for the current bag inventory for this wholesale distributor
    pub fn get_entity_bag_inventory(&mut self) -> &u8 

# set function for the current case inventory
    pub fn set_entity_case_inventory(&mut self, case_inventory: u8) 

# get function for the current case inventory
    pub fn get_entity_case_inventory(&mut self) -> &u8 

# set function for the current allowed case inventory
    pub fn set_entity_allowed_inventory_cases(&mut self, allowed_inventory_cases: u8) 

# get function for the current allowed case inventory
     pub fn get_entity_allowed_inventory_cases(&mut self) -> &u8 

# the eventual reason for the encription is to keep the wholesale distributor account id encripted on and contract that can be publicaly accessed

# set function to store the encripted AccountId for the entity owner for verification
    pub fn set_entity_owner(&mut self, owner: String) 
 
# get function to get the stored encripted owner Account Id
    pub fn get_entity_owner(&mut self) -> std::string::String 
  
# This funciton is to verify an account id is the same as the encripted id

    pub fn check_owner(&mut self, owner: String) -> bool 

Current Public Functions for EndCustomerOrder 
#############################################
#  More functions will be added in the future for Order maintinance, status changes and order processing

# Initialization function

        pub fn new(
            ec_account: AccountId,
            ec_customer_id: u8,
            ec_order_id: u64,
            ec_bags_ordered: u8,
            ec_cases_ordered: u8,
            ec_bag_purchase_price: f64,
            ec_cases_purchase_price: f64,
            ec_shipping_address1: String,
            ec_shipping_address2: String,
            ec_shipping_address3: String,
            ec_apt_or_unit: String,
            ec_state: String,
            ec_zip: String,
            ec_shipping_cost: f64,
            ec_order_status: String,) -> Self

