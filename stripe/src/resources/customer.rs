use client::Client;
use error::Error;
use ids::PaymentSourceId;
use params::{Identifiable, List, Metadata, RangeQuery, Timestamp};
use resources::{Address, Currency, Deleted, Discount, PaymentSourceParams, Source, Subscription};
use serde_qs as qs;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerShippingDetails {
    pub address: Address,
    pub name: String,
    pub phone: String,
}

/// The set of parameters that can be used when creating or updating a customer.
///
/// For more details see https://stripe.com/docs/api#create_customer and https://stripe.com/docs/api#update_customer.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CustomerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_vat_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<PaymentSourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParams<'a>>,
}

/// The set of parameters that can be used when listing customers.
///
/// For more details see https://stripe.com/docs/api#list_customers
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

/// The resource representing a Stripe customer.
///
/// For more details see https://stripe.com/docs/api#customers.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: String,
    pub account_balance: i64,
    pub business_vat_id: Option<String>,
    pub created: Option<Timestamp>,
    pub currency: Option<Currency>,
    pub default_source: Option<PaymentSourceId>,
    pub delinquent: bool,
    pub desc: Option<String>,
    pub discount: Option<Discount>,
    pub email: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub shipping: Option<CustomerShippingDetails>,
    pub sources: List<Source>,
    pub subscriptions: List<Subscription>,
}

impl Customer {
    /// Creates a new customer.
    ///
    /// For more details see https://stripe.com/docs/api#create_customer.
    pub fn create(client: &Client, params: CustomerParams) -> Result<Customer, Error> {
        client.post("/customers", params)
    }

    /// Retrieves the details of a customer.
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_customer.
    pub fn retrieve(client: &Client, customer_id: &str) -> Result<Customer, Error> {
        client.get(&format!("/customers/{}", customer_id))
    }

    /// Updates a customer's properties.
    ///
    /// For more details see https://stripe.com/docs/api#update_customer.
    pub fn update(
        client: &Client,
        customer_id: &str,
        params: CustomerParams,
    ) -> Result<Customer, Error> {
        client.post(&format!("/customers/{}", customer_id), params)
    }

    /// Deletes a customer.
    ///
    /// For more details see https://stripe.com/docs/api#delete_customer.
    pub fn delete(client: &Client, customer_id: &str) -> Result<Deleted, Error> {
        client.delete(&format!("/customers/{}", customer_id))
    }

    /// List customers.
    ///
    /// For more details see https://stripe.com/docs/api#list_customers.
    pub fn list(client: &Client, params: CustomerListParams) -> Result<List<Customer>, Error> {
        client.get(&format!("/customers?{}", qs::to_string(&params)?))
    }
}

impl Identifiable for Customer {
    fn id(&self) -> &str {
        &self.id
    }
}
