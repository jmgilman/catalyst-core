window.SIDEBAR_ITEMS = {"enum":[["ReadError",""],["WriteError",""]],"trait":[["Block","Block property"],["BlockDate","A trait representing block dates."],["BlockId","Trait identifying the block identifier type. TODO: add a Readable trait bound"],["ChainLength",""],["Deserialize","Define that an object that can be read from an `std::io::Read` object."],["DeserializeFromSlice","Define that an object can be read from a byte slice. This trait is implemented for all `Deserialize` implementors by default. The default implementation can be overridden if the user is sure they can benefit from slice-specific functions of `Codec`."],["Fragment","A fragment is some item contained in a block, such as a transaction, a delegation-related certificate, an update proposal, and so on. Fragments can be serialized (so that they can be concatenated to form a binary block( and have a unique ID (typically the hash of their serialization). TODO: add a Readable trait bound"],["FragmentId","Trait identifying the fragment identifier type. TODO: add a Readable trait bound"],["FromStr","Defines the way to parse the object from a UTF-8 string."],["HasFragments","Accessor to fragments within a block."],["HasHeader","Access to the block header."],["Header","Trait identifying the block header type."],["Serialize","Define that an object can be written to an `std::io::Write` object."],["Transaction","define a transaction within the blockchain. This transaction can be used for the UTxO model. However it can also be used for any other elements that the blockchain has (a transaction type to add Stacking Pools and so on…)."],["TransactionId","Trait identifying the transaction identifier type."]]};