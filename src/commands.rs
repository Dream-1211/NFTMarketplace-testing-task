use serde::{Deserialize, Serialize};

/// A batch of order instructions.
/// This command accepts only the following batches of commands
/// and will be processed in the following order:
/// - OrderCancellation
/// - OrderAmendment
/// - OrderSubmission
/// The total amount of commands in the batch across all three lists of
/// instructions is restricted by the following network parameter:
/// "spam.protection.max.batchSize"
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchMarketInstructions {
    /// A list of order cancellations to be processed sequentially
    pub cancellations: Vec<OrderCancellation>,
    /// A list of order amendments to be processed sequentially
    pub amendments: Vec<OrderAmendment>,
    /// A list of order submissions to be processed sequentially
    pub submissions: Vec<OrderSubmission>,
}

/// Time In Force for an order
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimeInForce {
    /// Default value for TimeInForce, can be valid for an amend
    #[serde(rename = "TIME_IN_FORCE_UNSPECIFIED")]
    Unspecified = 0,
    /// Good until cancelled, the order trades any amount and as much as possible
    /// and remains on the book until it either trades completely or is cancelled
    #[serde(rename = "TIME_IN_FORCE_GTC")]
    Gtc = 1,
    /// Good until specified time, this order type trades any amount and as much as possible
    /// and remains on the book until it either trades completely, is cancelled, or expires at a set time
    /// NOTE: this may in future be multiple types or have sub types for orders that provide different ways of specifying expiry
    #[serde(rename = "TIME_IN_FORCE_GTT")]
    Gtt = 2,
    /// Immediate or cancel, the order trades any amount and as much as possible
    /// but does not remain on the book (whether it trades or not)
    #[serde(rename = "TIME_IN_FORCE_IOC")]
    Ioc = 3,
    /// Fill or kill, The order either trades completely (remainingSize == 0 after adding)
    /// or not at all, does not remain on the book if it doesn't trade
    #[serde(rename = "TIME_IN_FORCE_FOK")]
    Fok = 4,
    /// Good for auction, this order is only accepted during an auction period
    #[serde(rename = "TIME_IN_FORCE_GFA")]
    Gfa = 5,
    /// Good for normal, this order is only accepted during normal trading (that can be continuous trading or frequent batched auctions)
    #[serde(rename = "TIME_IN_FORCE_GFN")]
    Gfn = 6,
}

/// Type values for an order
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    /// Default value, always invalid
    #[serde(rename = "TYPE_UNSPECIFIED")]
    Unspecified = 0,
    /// Used for Limit orders
    #[serde(rename = "TYPE_LIMIT")]
    Limit = 1,
    /// Used for Market orders
    #[serde(rename = "TYPE_MARKET")]
    Market = 2,
    /// Used for orders where the initiating party is the network (with distressed parties)
    #[serde(rename = "TYPE_NETWORK")]
    Network = 3,
}

/// A side relates to the direction of an order, to Buy, or Sell
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    /// Default value, always invalid
    #[serde(rename = "SIDE_UNSPECIFIED")]
    Unspecified = 0,
    /// Buy order
    #[serde(rename = "SIDE_BUY")]
    Buy = 1,
    /// Sell order
    #[serde(rename = "SIDE_SELL")]
    Sell = 2,
}

/// A pegged reference defines which price point a pegged order is linked to - meaning
/// the price for a pegged order is calculated from the value of the reference price point
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PeggedReference {
    /// Default value for PeggedReference, no reference given
    #[serde(rename = "PEGGED_REFERENCE_UNSPECIFIED")]
    Unspecified = 0,
    /// Mid price reference
    #[serde(rename = "PEGGED_REFERENCE_MID")]
    Mid = 1,
    /// Best bid price reference
    #[serde(rename = "PEGGED_REFERENCE_BEST_BID")]
    BestBid = 2,
    /// Best ask price reference
    #[serde(rename = "PEGGED_REFERENCE_BEST_ASK")]
    BestAsk = 3,
}

/// Pegged orders are limit orders where the price is specified in the form REFERENCE +/- OFFSET
/// They can be used for any limit order that is valid during continuous trading
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeggedOrder {
    /// The price point the order is linked to
    pub reference: PeggedReference,
    /// Offset from the price reference
    pub offset: String,
}

/// An order submission is a request to submit or create a new order on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderSubmission {
    /// Market identifier for the order, required field
    pub market_id: String,
    /// Price for the order, the price is an integer, for example `123456` is a correctly
    /// formatted price of `1.23456` assuming market configured to 5 decimal places,
    /// , required field for limit orders, however it is not required for market orders
    pub price: String,
    /// Size for the order, for example, in a futures market the size equals the number of units, cannot be negative
    pub size: u64,
    /// Side for the order, e.g. SIDE_BUY or SIDE_SELL, required field
    /// - See `Side`
    pub side: Side,
    /// Time in force indicates how long an order will remain active before it is executed or expires, required field
    /// - See `Order.TimeInForce`
    pub time_in_force: TimeInForce,
    /// Timestamp for when the order will expire, in nanoseconds since the epoch,
    /// required field only for `Order.TimeInForce`.TIME_IN_FORCE_GTT`
    /// - See `VegaTimeResponse`.`timestamp`
    pub expires_at: i64,
    /// Type for the order, required field - See `Order.Type`
    pub r#type: OrderType,
    /// Reference given for the order, this is typically used to retrieve an order submitted through consensus, currently
    /// set internally by the node to return a unique reference identifier for the order submission
    pub reference: String,
    /// Used to specify the details for a pegged order
    /// - See `PeggedOrder`
    pub pegged_order: Option<PeggedOrder>,
}
/// An order cancellation is a request to cancel an existing order on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCancellation {
    /// Unique identifier for the order (set by the system after consensus), required field
    pub order_id: String,
    /// Market identifier for the order, required field
    pub market_id: String,
}
/// An order amendment is a request to amend or update an existing order on Vega
///
/// The `orderID`, `partyID` and `marketID` fields are used for looking up the order only and cannot be amended by this command
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendment {
    /// Order identifier, this is required to find the order and will not be updated, required field
    pub order_id: String,
    /// Market identifier, this is required to find the order and will not be updated
    pub market_id: String,
    /// Amend the price for the order, if the Price value is set, otherwise price will remain unchanged - See \[`Price`\](#vega.Price)
    pub price: Option<String>,
    /// Amend the size for the order by the delta specified:
    /// - To reduce the size from the current value set a negative integer value
    /// - To increase the size from the current value, set a positive integer value
    /// - To leave the size unchanged set a value of zero
    pub size_delta: i64,
    /// Amend the expiry time for the order, if the Timestamp value is set, otherwise expiry time will remain unchanged
    /// - See \[`VegaTimeResponse`\](#api.VegaTimeResponse).`timestamp`
    pub expires_at: Option<i64>,
    /// Amend the time in force for the order, set to TIME_IN_FORCE_UNSPECIFIED to remain unchanged
    /// - See \[`TimeInForce`\](#api.VegaTimeResponse).`timestamp`
    pub time_in_force: TimeInForce,
    /// Amend the pegged order offset for the order
    pub pegged_offset: String,
    /// Amend the pegged order reference for the order
    /// - See \[`PeggedReference`\](#vega.PeggedReference)
    pub pegged_reference: i32,
}

/// Vote value
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum VoteValue {
    /// Default value, always invalid
    #[serde(rename = "VALUE_UNSPECIFIED")]
    Unspecified = 0,
    /// A vote against the proposal
    #[serde(rename = "VALUE_NO")]
    No = 1,
    /// A vote in favour of the proposal
    #[serde(rename = "VALUE_YES")]
    Yes = 2,
}

/// A command to submit a new vote for a governance
/// proposal.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct VoteSubmission {
    /// The ID of the proposal to vote for.
    pub proposal_id: String,
    /// The actual value of the vote
    pub value: VoteValue,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Command {
    BatchMarketInstructions(BatchMarketInstructions),
    OrderSubmission(OrderSubmission),
    OrderCancellation(OrderCancellation),
    OrderAmendment(OrderAmendment),
    VoteSubmission(VoteSubmission),
}

impl From<VoteSubmission> for Command {
    fn from(vs: VoteSubmission) -> Self {
        Command::VoteSubmission(vs)
    }
}
