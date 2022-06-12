//  Background and description
//  +++++++++++++++++++++++++++++
//# Rust Smart Contract a non-profit and it's Inventory and Orders and their status
//###################################################################################
//
//  This is for Hackathon submition
//
//  In the long-run this contract will be accessed by two different front
//  end websites. 
//  
//1.  The application portal for a non-profit supplier that in this example is a manufacturor of 
//Bags and cases of bags of product. 
//    
//2.  The web site for a non profit that sells the product in this example on their website and pre-purchases inventory that is held by the manufacturer and drop shipped to customers that are making purchases as a donation those orders are added to the contract.
//
//Example Description
//###################
//
//  1.  The non-profit first applies for a non-profit account with the manufacturer via the  manufacturers website.  The contract is deployed with the non-profit contract with a status of "Applicant".
//  
//  2.  The manufacturer then approves the non-profit organizaiton.  The Status is then changed to "Approved".
//  
//  3.  The non-profit then pre-orders inventory of bags and cases of product that the manufacturer   can hold and drop-ship as orders are made from the non-profit website.
//
//  4.  Orders are then placed on the non-profit website and paid for either in NEAR or by credit card" payment. This will be implemented in a later version of this contract.
//  
//  5.  The order is then added to the list of orders for the non-profit.  The status of the order at that time will be "ordered".
//  
//  6.  The manufacturer then drop-ships the order to the address on the order that was added to the contract.
//  
//  7.  The status of the order is then updated to "Shipped".
//  
//  8.  Additional status changes for tracking delivery of the product can also be added to the order record.
//  
//  9.  In future versions of this contract I invision adding a "Factory" contract and adding a collection of    wholesale dostributors to this contract, or creating multiple instances of this contract that exist only as long as the wholesale customer/distributor is active.
//
//  Also in the future we will probably need to ad some mechinism to store the end customer or person donating's invormation so that it is not visible on the blockchain.  maybe create subaccounts for each customer with address information in those accounts.  Anyway this needs to be addressed
//  
//  10. The unit test for this contract creates the contract puts the non-profit in "Applicant" status, updates them to "Approved" status then adds an order to the order list.  Also all get and set methods are tested in addition to an encription method for storing account information encripted.  Then verifying that the account Id matches the encripted value stored.
//
//  11. This initial version is to track the transaction information and Drop shippment of product for the non-profit.  Future functionality will include payment options this will most likely be in the form of taking payment by either credit card or crypto tokens and converting to stablecoin to transfer payment to supplier/manufacturer
//  12. Interfaces for both the supplier and the non-profit web sites/ user Interfaces or Apps on moble phones will be the data entry and transaction source.
// dependencies 

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;
use crate::borsh::maybestd::collections::HashMap;
use near_sdk::{env, near_bindgen, PanicOnDefault};
use serde::Serializer;
use serde::Serialize;
use serde::ser::SerializeStruct;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::str;
use std::fmt;
    
// Customer Order Structure
// A HashMap of Orders using this structure is in the Contract for the non-profit That is unique to each contract instance.
    #[near_bindgen]
    #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
    pub struct EndCustomerOrder{
        ec_ws_account: AccountId,  // Account Id of the end customer that purchases from the wholesale distributor
        ec_ws_customer_id: u8, // The Customer Id assigned by the wholesale distributor to the end customer
        ec_ws_order_id: u64, // The Order Identification number assigned by the wholesale distributor
        ec_ws_bags_ordered: u8, // Bags of product ordered
        ec_ws_cases_ordered: u8, // Cases of product ordered
        ec_ws_bag_purchase_price: f64, // The purchase price of each bag on this order
        ec_ws_cases_purchase_price: f64, // The purchase price of each case of product ordered
        ec_ws_shipping_address1: String, // Drop ship shipping address line 1
        ec_ws_shipping_address2: String, // Drop ship shipping address line 2
        ec_ws_shipping_address3: String, // Drop ship shipping address line 3
        ec_ws_apt_or_unit: String, // The appartment or unit of the Drop ship address
        ec_ws_ec_state: String, // the State of the Drop ship address
        ec_ws_zip: String, // the zipcode of the Drop ship address
        ec_ws_shipping_cost: f64, // The shipping cost of the drop ship
        ec_ws_order_status: String, // The order status  either "ordered" or "shipped"
    }

    //  structure initialization for the EndCustomerOrder Structure 
    impl EndCustomerOrder {
        // ADD CONTRACT METHODS HERE
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
            {
                Self{
                    ec_ws_account: ec_account,
                    ec_ws_customer_id: ec_customer_id,
                    ec_ws_order_id: ec_order_id,
                    ec_ws_bags_ordered: ec_bags_ordered,
                    ec_ws_cases_ordered: ec_cases_ordered,
                    ec_ws_bag_purchase_price: ec_bag_purchase_price,
                    ec_ws_cases_purchase_price:  ec_cases_purchase_price,
                    ec_ws_shipping_address1: ec_shipping_address1,
                    ec_ws_shipping_address2: ec_shipping_address2,
                    ec_ws_shipping_address3: ec_shipping_address3,
                    ec_ws_apt_or_unit: ec_apt_or_unit,
                    ec_ws_ec_state: ec_state,
                    ec_ws_zip: ec_zip,
                    ec_ws_shipping_cost: ec_shipping_cost,
                    ec_ws_order_status: ec_order_status,
                }
            }
    
            pub fn get_customer_id(&mut self) -> u8{
                self.ec_ws_customer_id
            }
        }
    
    // Serialization method for the EndCustomerOrder struct
    impl Serialize for EndCustomerOrder {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // 3 is the number of fields in the struct.
            let mut state = serializer.serialize_struct("EndCustomerOrder", 13)?;
            state.serialize_field("ec_ws_account", &self.ec_ws_account)?;
            state.serialize_field("ec_ws_customer_id", &self.ec_ws_customer_id)?;
            state.serialize_field("ec_ws_order_id",&self.ec_ws_order_id)?;
            state.serialize_field("ec_ws_bags_ordered", &self.ec_ws_bags_ordered)?;
            state.serialize_field("ec_ws_cases_ordered", &self.ec_ws_cases_ordered)?;
            state.serialize_field("ec_ws_bag_purchase_price", &self.ec_ws_bag_purchase_price)?;
            state.serialize_field("ec_ws_cases_purchase_price", &self.ec_ws_cases_purchase_price)?;
            state.serialize_field("ec_ws_shipping_address1", &self.ec_ws_shipping_address1)?;
            state.serialize_field("ec_ws_shipping_address2", &self.ec_ws_shipping_address2)?;
            state.serialize_field("ec_ws_shipping_address3", &self.ec_ws_shipping_address3)?;
            state.serialize_field("ec_ws_apt_or_unit", &self.ec_ws_apt_or_unit)?;
            state.serialize_field("ec_ws_ec_state", &self.ec_ws_ec_state)?;
            state.serialize_field("ec_ws_zip", &self.ec_ws_zip)?;
            state.serialize_field("ec_ws_shipping_cost", &self.ec_ws_shipping_cost)?;
            state.serialize_field("ec_ws_order_status", &self.ec_ws_order_status)?;
            state.end()
        }
    }
    
    // Deserialization of the EndCustomerOrder Structure
    impl<'de> Deserialize<'de> for EndCustomerOrder {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            enum Field { 
                EcWsAccount,
                EcWsCustomerId,
                EcWsOrderId,
                EcWsBagsOrdered,
                EcWsCasesOrdered,
                EcWsBagPurchasePrice,
                EcWsCasesPurchasePrice,
                EcWsShippingAddress1,
                EcWsShippingAddress2,
                EcWsShippingAddress3,
                EcWsAptOrUnit,
                EcWsEcState,
                EcWsZip,
                EcWsShippingCost,
                EcWsOrderStatus
            }
    
            impl<'de> Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    struct FieldVisitor;
    
                    impl<'de> Visitor<'de> for FieldVisitor {
                        type Value = Field;
    
                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str(
                                "'ec_ws_account' or
                                       'ec_ws_customer_id' or
                                       'ec_ws_order_id' or
                                       'ec_ws_bags_ordered' or
                                       'ec_ws_cases_ordered' or
                                       'ec_ws_bag_purchase_price' or
                                       'ec_ws_cases_purchase_price' or
                                       'ec_ws_shipping_address1' or
                                       'ec_ws_shipping_address2' or
                                       'ec_ws_shipping_address3' or
                                       'ec_ws_apt_or_unit' or
                                       'ec_ws_ec_state' or
                                       'ec_ws_zip' or
                                       'ec_ws_shipping_cost' or
                                       'ec_ws_order_status'")
                        }
    
                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                        {
                            match value {
                                "ec_ws_account" => Ok(Field::EcWsAccount),
                                "ec_ws_customer_id" => Ok(Field::EcWsCustomerId),
                                "ec_ws_order_id" => Ok(Field::EcWsOrderId),
                                "ec_ws_bags_ordered" => Ok(Field::EcWsBagsOrdered),
                                "ec_ws_cases_ordered" => Ok(Field::EcWsCasesOrdered),
                                "ec_ws_bag_purchase_price" => Ok(Field::EcWsBagPurchasePrice),
                                "ec_ws_cases_purchase_price" => Ok(Field::EcWsCasesPurchasePrice),
                                "ec_ws_shipping_address1" => Ok(Field::EcWsShippingAddress1),
                                "ec_ws_shipping_address2" => Ok(Field::EcWsShippingAddress2),
                                "ec_ws_shipping_address3" => Ok(Field::EcWsShippingAddress3),
                                "ec_ws_apt_or_unit" => Ok(Field::EcWsAptOrUnit),
                                "ec_ws_ec_state" => Ok(Field::EcWsEcState),
                                "ec_ws_zip" => Ok(Field::EcWsZip),
                                "ec_ws_ec_shipping_cost" => Ok(Field::EcWsShippingCost),
                                "ec_ws_ec_order_status" => Ok(Field::EcWsOrderStatus),
                                _ => Err(de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
    
                    deserializer.deserialize_identifier(FieldVisitor)
                }
            }
    
            struct EndCustomerOrderVisitor;
    
            impl<'de> Visitor<'de> for EndCustomerOrderVisitor {
                type Value = EndCustomerOrder;
    
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct EndCustomerOrder")
                }
    
                fn visit_seq<V>(self, mut seq: V) -> Result<EndCustomerOrder, V::Error>
                where
                    V: SeqAccess<'de>,
                {
                    let ec_ws_account = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let ec_ws_customer_id = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_order_id = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_bags_ordered = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_cases_ordered = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_bag_purchase_price = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_cases_purchase_price = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_shipping_address1 = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_shipping_address2 = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_shipping_address3 = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_apt_or_unit = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_ec_state = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_zip = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_shipping_cost = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let ec_ws_order_status = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                        Ok(EndCustomerOrder::new(ec_ws_account, 
                                     ec_ws_customer_id,
                                     ec_ws_order_id,
                                     ec_ws_bags_ordered,
                                     ec_ws_cases_ordered,
                                     ec_ws_bag_purchase_price,
                                     ec_ws_cases_purchase_price,
                                     ec_ws_shipping_address1,
                                     ec_ws_shipping_address2,
                                     ec_ws_shipping_address3,
                                     ec_ws_apt_or_unit,
                                     ec_ws_ec_state,
                                     ec_ws_zip,
                                     ec_ws_shipping_cost,
                                     ec_ws_order_status
                                    ))
                }
    
                fn visit_map<V>(self, mut map: V) -> Result<EndCustomerOrder, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut ec_ws_account = None;
                    let mut ec_ws_customer_id = None;
                    let mut ec_ws_order_id = None;
                    let mut ec_ws_bags_ordered = None;
                    let mut ec_ws_cases_ordered = None;
                    let mut ec_ws_bag_purchase_price   = None;
                    let mut ec_ws_cases_purchase_price   = None;
                    let mut ec_ws_shipping_address1 = None;
                    let mut ec_ws_shipping_address2 = None;
                    let mut ec_ws_shipping_address3 = None;
                    let mut ec_ws_apt_or_unit = None;
                    let mut ec_ws_ec_state = None;
                    let mut ec_ws_zip = None;
                    let mut ec_ws_shipping_cost = None;
                    let mut ec_ws_order_status = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::EcWsAccount => {
                                if ec_ws_account.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_account"));
                                }
                                ec_ws_account = Some(map.next_value()?);
                            }
                            Field::EcWsCustomerId => {
                                if ec_ws_customer_id.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_customer_id"));
                                }
                                ec_ws_customer_id = Some(map.next_value()?);
                            }
                            Field::EcWsOrderId => {
                                if ec_ws_order_id.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_order_id"));
                                }
                                ec_ws_order_id = Some(map.next_value()?);
                            }
                            Field::EcWsBagsOrdered => {
                                if ec_ws_bags_ordered.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_bags_ordered"));
                                }
                                ec_ws_bags_ordered = Some(map.next_value()?);
                            }
                            Field::EcWsCasesOrdered => {
                                if ec_ws_cases_ordered.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_cases_ordered"));
                                }
                                ec_ws_cases_ordered = Some(map.next_value()?);
                            }
                            Field::EcWsBagPurchasePrice => {
                                if ec_ws_bag_purchase_price.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_bag_purchase_price"));
                                }
                                ec_ws_bag_purchase_price = Some(map.next_value()?);
                            }
                            Field::EcWsCasesPurchasePrice => {
                                if ec_ws_cases_purchase_price.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_cases_purchase_price"));
                                }
                                ec_ws_cases_purchase_price = Some(map.next_value()?);
                            }
                            Field::EcWsShippingAddress1 => {
                                if ec_ws_shipping_address1.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_shipping_address1"));
                                }
                                ec_ws_shipping_address1 = Some(map.next_value()?);
                            }
                            Field::EcWsShippingAddress2 => {
                                if ec_ws_shipping_address2.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_shipping_address2"));
                                }
                                ec_ws_shipping_address2 = Some(map.next_value()?);
                            }
                            Field::EcWsShippingAddress3 => {
                                if ec_ws_shipping_address3.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_shipping_address3"));
                                }
                                ec_ws_shipping_address3 = Some(map.next_value()?);
                            }
                            Field::EcWsAptOrUnit => {
                                if ec_ws_apt_or_unit.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_apt_or_unit"));
                                }
                                ec_ws_apt_or_unit = Some(map.next_value()?);
                            }
                            Field::EcWsEcState => {
                                if ec_ws_ec_state.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_ec_state"));
                                }
                                ec_ws_ec_state = Some(map.next_value()?);
                            }
                            Field::EcWsZip => {
                                if ec_ws_zip.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_zip"));
                                }
                                ec_ws_zip = Some(map.next_value()?);
                            }
                            Field::EcWsShippingCost => {
                                if ec_ws_shipping_cost.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_shipping_cost"));
                                }
                                ec_ws_shipping_cost = Some(map.next_value()?);
                            }
                            Field::EcWsOrderStatus => {
                                if ec_ws_order_status.is_some() {
                                    return Err(de::Error::duplicate_field("ec_ws_order_status"));
                                }
                                ec_ws_order_status = Some(map.next_value()?);
                            }
                        }
                    }
                    let ec_ws_account = ec_ws_account.ok_or_else(|| de::Error::missing_field("mut ec_ws_account"))?;
                    let ec_ws_customer_id = ec_ws_customer_id.ok_or_else(|| de::Error::missing_field("ec_ws_customer_id"))?;
                    let ec_ws_order_id = ec_ws_order_id.ok_or_else(|| de::Error::missing_field("ec_ws_order_id"))?;
                    let ec_ws_bags_ordered = ec_ws_bags_ordered.ok_or_else(|| de::Error::missing_field("ec_ws_bags_ordered"))?;
                    let ec_ws_cases_ordered = ec_ws_cases_ordered.ok_or_else(|| de::Error::missing_field("ec_ws_cases_ordered"))?;
                    let ec_ws_bag_purchase_price = ec_ws_bag_purchase_price.ok_or_else(|| de::Error::missing_field("ec_ws_bag_purchase_price"))?;
                    let ec_ws_cases_purchase_price = ec_ws_cases_purchase_price.ok_or_else(|| de::Error::missing_field("ec_ws_bag_purchase_price"))?;
                    let ec_ws_shipping_address1 = ec_ws_shipping_address1.ok_or_else(|| de::Error::missing_field("ec_ws_shipping_address1"))?;
                    let ec_ws_shipping_address2 = ec_ws_shipping_address2.ok_or_else(|| de::Error::missing_field("ec_ws_shipping_address2"))?;
                    let ec_ws_shipping_address3 = ec_ws_shipping_address3.ok_or_else(|| de::Error::missing_field("ec_ws_shipping_address3"))?;
                    let ec_ws_apt_or_unit = ec_ws_apt_or_unit.ok_or_else(|| de::Error::missing_field("ec_ws_apt_or_unit"))?;
                    let ec_ws_ec_state = ec_ws_ec_state.ok_or_else(|| de::Error::missing_field("ec_ws_ec_state"))?;
                    let ec_ws_zip = ec_ws_zip.ok_or_else(|| de::Error::missing_field("ec_ws_zip"))?;
                    let ec_ws_shipping_cost = ec_ws_shipping_cost.ok_or_else(|| de::Error::missing_field("ec_ws_shipping_cost"))?;
                    let ec_ws_order_status = ec_ws_order_status.ok_or_else(|| de::Error::missing_field("ec_ws_order_status"))?;

                    Ok(EndCustomerOrder::new(ec_ws_account, 
                        ec_ws_customer_id,
                        ec_ws_order_id,
                        ec_ws_bags_ordered,
                        ec_ws_cases_ordered,
                        ec_ws_bag_purchase_price,
                        ec_ws_cases_purchase_price,
                        ec_ws_shipping_address1,
                        ec_ws_shipping_address2,
                        ec_ws_shipping_address3,
                        ec_ws_apt_or_unit,
                        ec_ws_ec_state,
                        ec_ws_zip,
                        ec_ws_shipping_cost,
                        ec_ws_order_status))
                }
            }
    
            const FIELDS: &'static [&'static str] 
                        = &["ec_ws_account", 
                            "ec_ws_customer_id",
                            "ec_ws_order_id",
                            "ec_ws_bags_ordered",
                            "ec_ws_cases_ordered",
                            "ec_ws_bag_purchase_price",
                            "ec_ws_cases_purchase_price",
                            "ec_ws_shipping_address1",
                            "ec_ws_shipping_address2",
                            "ec_ws_shipping_address3",
                            "ec_ws_apt_or_unit",
                            "ec_ws_ec_state",
                            "ec_ws_zip",
                            "ec_ws_shipping_cost",
                            "ec_ws_order_status"];
                            
            deserializer.deserialize_struct("EndCustomerOrder", FIELDS, EndCustomerOrderVisitor)
        }
    }
    

//Main contract Definition
//  One contract will exist for each non-profit
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract { //main contract
    // SETUP CONTRACT STATE
    ws_entity_account: String, // non-profit account
    ws_entity_main_account: AccountId, // Supplier AccountId
    ws_entity_name: String, //non-profit entity name
    ws_entity_status: String, //non-profit status: Currently either "Applicant" or "Approved"
    ws_entity_description: String, //non-profit entity description
    ws_entity_allowed_inventory_bags: u8,//The number of bags of inventory non-profit is allowed to have in inventory for dropship
    ws_entity_allowed_inventory_cases: u8, //the number of cases of product that the non-profit is allowed to inventory for dropship
    ws_bag_inventory: u8, // the number of bags currently in inventory for this non-profit
    ws_case_inventory: u8, // the number of cases currently in inventory for this non-profit
    ws_entity_owner: std::string::String, // the non-profit enchripted account
    ws_customer_orders : HashMap<u64, EndCustomerOrder>,// HashMap list of orders placed for dropshipment by customers of the non-profit
}

// Main contract implimentation

#[near_bindgen]
impl Contract{
    #[init]
    // Public functions 

    // Initialization function
    pub fn new() -> Self
        {
            let entity_account = "default.testnet";
            let entity_main_account = AccountId::new_unchecked("default.testnet".to_string());
            let entity_name = "";
            let entity_status = "Applicant";
            let entity_description = "";
            let entity_owner = "default.testnet";
            let entity_allowed_inventory_bags = 40 as u8;
            let entity_allowed_inventory_cases = 20 as u8;
            let entity_bag_inventory = 0;
            let entity_case_inventory = 0;
  
            Self{ws_entity_account : entity_account.to_string(),
                ws_entity_main_account: entity_main_account,
                ws_entity_name : entity_name.to_string(),
                ws_entity_status : entity_status.to_string(),
                ws_entity_description : entity_description.to_string(),
                ws_entity_owner : entity_owner.to_string(),
                ws_entity_allowed_inventory_bags : entity_allowed_inventory_bags,
                ws_entity_allowed_inventory_cases : entity_allowed_inventory_cases,
                ws_bag_inventory : entity_bag_inventory,
                ws_case_inventory : entity_case_inventory,
                ws_customer_orders : HashMap::new(),
            }  
        }
         
        // Add a customer order to the HashMap list of Orders for this non-profit    
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
        {
            // if the non-profits status is "Approved" the non-profit is allowed to receive new orders
            if self.ws_entity_status == "Approved"{
                let mut _new_order = EndCustomerOrder::new(
                    ec_account,
                    ec_customer_id,
                    ec_order_id,
                    ec_bags_ordered,
                    ec_cases_ordered,
                    ec_bag_purchase_price,
                    ec_cases_purchase_price,
                    ec_shipping_address1,
                    ec_shipping_address2,
                    ec_shipping_address3,
                    ec_apt_or_unit,
                    ec_state,
                    ec_zip,
                    ec_shipping_cost,
                    ec_order_status);
                    env::log_str("New Customer Order Created");

                    let _err = self.ws_customer_orders.insert(ec_order_id, _new_order);
                }
                else{
                env::log_str("New Customer Order Not Created, Status not Approved");
                }
            } 

    // get funtion for the HashMap Order List
    pub fn get_customer_orders(&self) -> &HashMap<u64, EndCustomerOrder>{
        return &self.ws_customer_orders;
    }

    // set function for the account mane
    pub fn set_entity_account(&mut self, account: String) {
        self.ws_entity_account = account;
    }

    // get function for the account name
    pub fn get_entity_account(&mut self) -> &String {
        return &self.ws_entity_account;
    }

    // set function for the main account Id
    pub fn set_entity_main_account(&mut self, account: AccountId) {
        self.ws_entity_main_account = account;
    }

    // get function for the account name
    pub fn get_entity_main_account(&mut self) -> &AccountId {
        return &self.ws_entity_main_account;
    }

    // set function for the non-profit entity name
    pub fn set_entity_name(&mut self, name: String) {
        self.ws_entity_name = name;
    }
    
    // get function for the non-profit entity name
    pub fn get_entity_name(&mut self) -> &String {
        return &self.ws_entity_name;
    }
    
    // set function for non-profit status
    pub fn set_entity_status(&mut self, status: String) {
        // If the non-profit is currently and "Applicant" and a request is being made 
        // Change the non-profit status to Approved then allow the status change.
        if self.ws_entity_status == "Applicant"{
            if status == "Approved"{
                self.ws_entity_status = status;
                env::log_str("Entity Status Set to Approved");
            }
        }
        else {
            // if the non-profit is set to status "nostat"
            if self.ws_entity_status == "nostat"{
                // if a new status of Applicant is being requested then Allow status change to 
                // be changed to "Applicant"
                if status == "Applicant"{
                    self.ws_entity_status = status;
                    env::log_str("Entity Status Set to Applicant");
                }
            }
            else{
                // if request is being made to change status to Applicant
                if status == "Applicant"{
                    // Then if the current status is Approved Then allow the status change
                    if self.ws_entity_status != "Approved" {
                        self.ws_entity_status = status;
                        env::log_str("Entity Status Set to Applicant");
                    }
                }
                else {
                    // if requested status is not "Applicant" and is not allready "Approved"
                    // then the status is an unknown status and the current status is set to "nostat"
                    if self.ws_entity_status != "Approved"{
                        self.ws_entity_status = "nostat".to_string();
                        env::log_str("Unknown Status, Entity Status Set to nostat");
                    }
                    // if the status is "Approved" then status override of status "Approved" is not allowed
                    else {
                        env::log_str("Entity Status already approved. Override of Status Approved not allowed");
                    }
                }
            }
            
        }
    }

    // get function for the non-profit entity status
    pub fn get_entity_status(&mut self) -> &String {
        return &self.ws_entity_status;
    }
    
    // set function for entity description
    pub fn set_entity_description(&mut self, description: String) {
        self.ws_entity_description = description;
    }

    // get function for the entity description
    pub fn get_entity_description(&mut self) -> &String {
        return &self.ws_entity_description;
    }
    
    // set function for the allowed number of bags to have in inventory
    pub fn set_entity_allowed_inventory_bags(&mut self, allowed_inventory_bags: u8) {
        self.ws_entity_allowed_inventory_bags = allowed_inventory_bags;
    }

    // get function for the allowed number of bags ti gave in inventory
    pub fn get_entity_allowed_inventory_bags(&mut self) -> &u8 {
        return &self.ws_entity_allowed_inventory_bags;
    }
    
    // set function for current bag inventory for this non-profit
    pub fn set_entity_bag_inventory(&mut self, bag_inventory: u8) {
        self.ws_bag_inventory = bag_inventory;
    }

    // get function for the current bag inventory for this non-profit
    pub fn get_entity_bag_inventory(&mut self) -> &u8 {
        return &self.ws_bag_inventory;
    }

    // set function for the current case inventory
    pub fn set_entity_case_inventory(&mut self, case_inventory: u8) {
        self.ws_case_inventory = case_inventory;
    }

    // get function for the current case inventory
    pub fn get_entity_case_inventory(&mut self) -> &u8 {
        return &self.ws_case_inventory;
    }

    // set function for the current allowed case inventory
    pub fn set_entity_allowed_inventory_cases(&mut self, allowed_inventory_cases: u8) {
        self.ws_entity_allowed_inventory_cases = allowed_inventory_cases;
    }

    // get function for the current allowed case inventory
    pub fn get_entity_allowed_inventory_cases(&mut self) -> &u8 {
        return &self.ws_entity_allowed_inventory_cases;
    }
    
    // the eventual reason for the encription is to keep the non-profit account id encripted on and contract that can be publicaly accessed
    // set function to store the encripted AccountId for the entity owner for verification
    pub fn set_entity_owner(&mut self, owner: String) {
        let hash_bytes = env::sha256(owner.as_bytes());
        let hash_string = hex::encode(hash_bytes);
        self.ws_entity_owner = hash_string;
    }

    // get function to get the stored encripted owner Account Id
    pub fn get_entity_owner(&mut self) -> std::string::String {
        return self.ws_entity_owner.to_string();
    }

    
// This funciton is to verify an account id is the same as the encripted id

    pub fn check_owner(&mut self, owner: String) -> bool 
    {
        let hash_bytes = env::sha256(owner.as_bytes());
        let hash_string = hex::encode(hash_bytes);
        let test_owner = hash_string;

        //println's for debuging purposes
        //println!("Entity owner is           : {:?}",self.ws_entity_owner);
        //println!("owner to test is          : {:?}",owner);
        //println!("owner to test encripted is : {:?}",test_owner);
        env::log_str(&("Owner to verify is: ".to_owned()+&owner+"\n"));
        
        if self.ws_entity_owner == test_owner 
        {
            env::log_str("Owner verified\n");
            return true;
        }
        else
        {
            env::log_str("Owner not verified\n");
            return false;
        }
    } 
}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package reg_entity -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // Constants used for Testing 
    const MANUFACTURER_ENTITY: &str = "alice.testnet";
    const ECCUSTOMER_NUMBER: u8 = 1;
    const WSXG_BAG_PRICE: f64 = 17.50;

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    // Function to unit test the encription of the Account Id
    fn debug_get_hash() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());

        // Using a unit test to rapidly debug and iterate
        let debug_owner = "secret owner";
        let debug_hash_bytes = env::sha256(debug_owner.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("\n Let's debug: {:?}", debug_hash_string);
    }


    #[test]
    // Function to test the contract to create a new contract with test values then to change the status of the
    // non-profit.  Then to create a new order for the non-profit and ad it to the 
    // non-profit Contract
    fn debug_check_contract() {
        // Get alice as an account ID
        let alice = AccountId::new_unchecked(MANUFACTURER_ENTITY.to_string());

        // Set up the testing context and unit test environment
        let context = get_context(alice);
        testing_env!(context.build());
    
        // Set up contract object with test data
        
        let entity_name = "Alices non-profit organization";
        let entity_status = "Applicant";
        let entity_description = "We Help make healthier food available to more people";
        let entity_allowed_inventory_bags = 40 as u8;
        let entity_allowed_inventory_cases = 20 as u8;
        let entity_bag_inventory = 25;
        let entity_case_inventory = 10;
        let _entity_customer_orders:HashMap<u64, EndCustomerOrder> = HashMap::new();
        
        let entity_owner:std::string::String ="alice.testnet".to_string();
        let _temp_id="alice.testnet".to_string();
        
        // Call the new method
        
        let mut contract = Contract::new();

        let owner_before = entity_owner.to_string();
        
        
        println!("Entity owner is before encription: {:?}",owner_before);
        
        contract.set_entity_owner(entity_owner.to_string());
        contract.set_entity_main_account(AccountId::new_unchecked(_temp_id));
        contract.set_entity_name(entity_name.to_string());
        contract.set_entity_status(entity_status.to_string());
        contract.set_entity_description(entity_description.to_string());
        
        contract.set_entity_owner(entity_owner.to_string());
        env::log_str("Owner set");
        
        contract.set_entity_allowed_inventory_bags(entity_allowed_inventory_bags as u8);
        contract.set_entity_allowed_inventory_cases(entity_allowed_inventory_cases as u8);
        contract.set_entity_bag_inventory(entity_bag_inventory as u8);
        contract.set_entity_case_inventory(entity_case_inventory as u8);
                
        let _owner_check = Contract::check_owner(&mut contract, owner_before);
        
        assert!(_owner_check == true, "Passed true owner test");
        
        let _test_owner = "wrongowner.testnet".to_string();
        
        let _not_owner_check = Contract::check_owner(&mut contract, _test_owner);
        assert!(_not_owner_check == false, "Passed not true owner test");
        
        println!("Test entity name is           : {:?}",Contract::get_entity_name(&mut contract));
        println!("Test entity status is         : {:?}",Contract::get_entity_status(&mut contract));
        println!("Test entity description is    : {:?}",Contract::get_entity_description(&mut contract));
        println!("Test entity allowed inv bags  : {:?}",Contract::get_entity_allowed_inventory_bags(&mut contract));
        println!("Test entity allowed inv cases : {:?}",Contract::get_entity_allowed_inventory_cases(&mut contract));
        println!("Test entity bag inventory     : {:?}",Contract::get_entity_bag_inventory(&mut contract));
        println!("Test entity case inventory    : {:?}",Contract::get_entity_case_inventory(&mut contract));
        println!("Test entity owner is          : {:?}",Contract::get_entity_owner(&mut contract));
        
        Contract::set_entity_status(&mut contract,"Approved".to_string());        
        
        let _test_logs:Vec<String> = get_logs();
        println!("Logs : {:?}",_test_logs);

        println!("Test entity status is         : {:?}",Contract::get_entity_status(&mut contract));
        
        Contract::set_entity_status(&mut contract,"Bad Status".to_string());        
        
        let _test_logs:Vec<String> = get_logs();
        println!("Logs : {:?}",_test_logs);

        println!("Test entity status is         : {:?}",Contract::get_entity_status(&mut contract));

        let end_customer_account:AccountId = AccountId::new_unchecked(MANUFACTURER_ENTITY.to_string());
        let end_customer_customer_id:u8 = ECCUSTOMER_NUMBER as u8;
        let end_customer_order_id:u64 = 1 as u64;
        let end_customer_bags_ordered:u8 = 2 as u8;
        let end_customer_cases_ordered:u8 = 0;
        let end_customer_bag_purchase_price:f64 = WSXG_BAG_PRICE;
        let end_customer_cases_purchase_price:f64 = 0 as f64;
        let end_customer_shipping_address1:String = "Shipping Address Line 1".to_string();
        let end_customer_shipping_address2:String = "Shipping Address Line 2".to_string();
        let end_customer_shipping_address3:String = "Shipping Address Line 3".to_string();
        let end_customer_apt_or_unit:String = "Apt c1234".to_string();
        let end_customer_state:String = "Texas".to_string();
        let end_customer_zip:String = "75123".to_string();
        let end_customer_shipping_cost:f64 = 8.34;
        let end_customer_order_status: String = "Ordered".to_string(); 
        Contract::set_customer_order(&mut contract,
            end_customer_account,
            end_customer_customer_id,
            end_customer_order_id,
            end_customer_bags_ordered,
            end_customer_cases_ordered,
            end_customer_bag_purchase_price,
            end_customer_cases_purchase_price,
            end_customer_shipping_address1,
            end_customer_shipping_address2,
            end_customer_shipping_address3,
            end_customer_apt_or_unit,
            end_customer_state,
            end_customer_zip,
            end_customer_shipping_cost,
            end_customer_order_status); 
            let _result:&HashMap<u64,EndCustomerOrder> = contract.get_customer_orders();
            
            if let Some(_end_customer_order) = _result.get(&end_customer_order_id){
                println!("End Customer AccountID           : {:?}",_end_customer_order.ec_ws_account);
                println!("End Customer ID                  : {:?}",_end_customer_order.ec_ws_customer_id);
                println!("End Order ID                     : {:?}",_end_customer_order.ec_ws_order_id);
                println!("End Customer Bags Ordered        : {:?}",_end_customer_order.ec_ws_bags_ordered);
                println!("End Customer Cases Ordered       : {:?}",_end_customer_order.ec_ws_cases_ordered);
                println!("End Customer Bag purchase Price  : {:?}",_end_customer_order.ec_ws_bag_purchase_price);
                println!("End Customer Case purchase Price : {:?}",_end_customer_order.ec_ws_cases_purchase_price);
                println!("End Customer Shipping address 1  : {:?}",_end_customer_order.ec_ws_shipping_address1);
                println!("End Customer Shipping address 2  : {:?}",_end_customer_order.ec_ws_shipping_address2);
                println!("End Customer Shipping address 3  : {:?}",_end_customer_order.ec_ws_shipping_address3);
                println!("End Customer Apt or Unit         : {:?}",_end_customer_order.ec_ws_apt_or_unit);
                println!("End Customer State               : {:?}",_end_customer_order.ec_ws_ec_state);
                println!("End Customer Zip                 : {:?}",_end_customer_order.ec_ws_zip);
                println!("End Customer Shipping Cost       : {:?}",_end_customer_order.ec_ws_shipping_cost);
                println!("End Customer Order Status        : {:?}",_end_customer_order.ec_ws_order_status);
                assert!(_end_customer_order.ec_ws_order_status == "Ordered", "Order status check correct.");
            }
      
        }     
}

