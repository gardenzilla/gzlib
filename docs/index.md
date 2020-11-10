# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [cash.proto](#cash.proto)
    - [CashLogItem](#cash.CashLogItem)
    - [GetBalanceRequest](#cash.GetBalanceRequest)
    - [GetBalanceResponse](#cash.GetBalanceResponse)
    - [ListRequest](#cash.ListRequest)
    - [ListResponse](#cash.ListResponse)
    - [PurchaseRequest](#cash.PurchaseRequest)
    - [PurchaseResponse](#cash.PurchaseResponse)
    - [TransactionRequest](#cash.TransactionRequest)
    - [TransactionResponse](#cash.TransactionResponse)
  
    - [Cash](#cash.Cash)
  
- [customer.proto](#customer.proto)
    - [AddUserRequest](#customer.AddUserRequest)
    - [AddUserResponse](#customer.AddUserResponse)
    - [Address](#customer.Address)
    - [CreateNewRequest](#customer.CreateNewRequest)
    - [CreateNewResponse](#customer.CreateNewResponse)
    - [CustomerObj](#customer.CustomerObj)
    - [CustomerUpdateObj](#customer.CustomerUpdateObj)
    - [GetAllResponse](#customer.GetAllResponse)
    - [GetByIdRequest](#customer.GetByIdRequest)
    - [GetByIdResponse](#customer.GetByIdResponse)
    - [RemoveUserRequest](#customer.RemoveUserRequest)
    - [RemoveUserResponse](#customer.RemoveUserResponse)
    - [UpdateByIdRequest](#customer.UpdateByIdRequest)
    - [UpdateByIdResponse](#customer.UpdateByIdResponse)
  
    - [Customer](#customer.Customer)
  
- [email.proto](#email.proto)
    - [EmailRequest](#email.EmailRequest)
  
    - [Email](#email.Email)
  
- [prelude.proto](#prelude.proto)
    - [DateTime](#prelude.DateTime)
    - [FloatValue](#prelude.FloatValue)
    - [UserId](#prelude.UserId)
  
- [product.proto](#product.proto)
    - [CreateNewRequest](#product.CreateNewRequest)
    - [CreateNewResponse](#product.CreateNewResponse)
    - [GetAllResponse](#product.GetAllResponse)
    - [GetByIdRequest](#product.GetByIdRequest)
    - [GetByIdResponse](#product.GetByIdResponse)
    - [IsSkuRequest](#product.IsSkuRequest)
    - [IsSkuResponse](#product.IsSkuResponse)
    - [ProductObj](#product.ProductObj)
    - [ProductUpdateObj](#product.ProductUpdateObj)
    - [SKU](#product.SKU)
    - [UpdateByIdRequest](#product.UpdateByIdRequest)
    - [UpdateByIdResponse](#product.UpdateByIdResponse)
  
    - [Product](#product.Product)
  
- [source.proto](#source.proto)
    - [PriceHistory](#source.PriceHistory)
    - [PriceHistoryItem](#source.PriceHistoryItem)
    - [Todo](#source.Todo)
  
    - [Source](#source.Source)
  
- [upl.proto](#upl.proto)
    - [CreateNewRequest](#upl.CreateNewRequest)
    - [CreateNewResponse](#upl.CreateNewResponse)
    - [GetUplRequest](#upl.GetUplRequest)
    - [GetUplResponse](#upl.GetUplResponse)
    - [MoveRequest](#upl.MoveRequest)
    - [MoveResponse](#upl.MoveResponse)
    - [Upl](#upl.Upl)
    - [UplByPlaceRequest](#upl.UplByPlaceRequest)
    - [UplHistoryItem](#upl.UplHistoryItem)
  
    - [UplStore](#upl.UplStore)
  
- [user.proto](#user.proto)
    - [AllUserRequest](#user.AllUserRequest)
    - [CreateNewRequest](#user.CreateNewRequest)
    - [CreateNewResponse](#user.CreateNewResponse)
    - [GetAllResponse](#user.GetAllResponse)
    - [GetByIdRequest](#user.GetByIdRequest)
    - [GetByIdResponse](#user.GetByIdResponse)
    - [IsUserRequest](#user.IsUserRequest)
    - [IsUserResponse](#user.IsUserResponse)
    - [LoginRequest](#user.LoginRequest)
    - [LoginResponse](#user.LoginResponse)
    - [NewPasswordRequest](#user.NewPasswordRequest)
    - [NewPasswordResponse](#user.NewPasswordResponse)
    - [ResetPasswordRequest](#user.ResetPasswordRequest)
    - [ResetPasswordResponse](#user.ResetPasswordResponse)
    - [UpdateByIdRequest](#user.UpdateByIdRequest)
    - [UpdateByIdResponse](#user.UpdateByIdResponse)
    - [UserObj](#user.UserObj)
  
    - [User](#user.User)
  
- [Scalar Value Types](#scalar-value-types)



<a name="cash.proto"></a>
<p align="right"><a href="#top">Top</a></p>

## cash.proto



<a name="cash.CashLogItem"></a>

### CashLogItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| date_created | [string](#string) |  |  |
| created_by | [string](#string) |  |  |
| amount | [int32](#int32) |  |  |
| kind | [string](#string) |  |  |






<a name="cash.GetBalanceRequest"></a>

### GetBalanceRequest







<a name="cash.GetBalanceResponse"></a>

### GetBalanceResponse







<a name="cash.ListRequest"></a>

### ListRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| from | [string](#string) |  |  |
| till | [string](#string) |  |  |






<a name="cash.ListResponse"></a>

### ListResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| log | [CashLogItem](#cash.CashLogItem) | repeated |  |






<a name="cash.PurchaseRequest"></a>

### PurchaseRequest







<a name="cash.PurchaseResponse"></a>

### PurchaseResponse







<a name="cash.TransactionRequest"></a>

### TransactionRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| created_by | [string](#string) |  |  |
| amount | [int32](#int32) |  |  |






<a name="cash.TransactionResponse"></a>

### TransactionResponse






 

 

 


<a name="cash.Cash"></a>

### Cash


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Purchase | [PurchaseRequest](#cash.PurchaseRequest) | [PurchaseResponse](#cash.PurchaseResponse) |  |
| Transaction | [TransactionRequest](#cash.TransactionRequest) | [TransactionResponse](#cash.TransactionResponse) |  |
| List | [ListRequest](#cash.ListRequest) | [ListResponse](#cash.ListResponse) |  |
| GetBalance | [GetBalanceRequest](#cash.GetBalanceRequest) | [GetBalanceResponse](#cash.GetBalanceResponse) |  |

 



<a name="customer.proto"></a>
<p align="right"><a href="#top">Top</a></p>

## customer.proto



<a name="customer.AddUserRequest"></a>

### AddUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customer_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |






<a name="customer.AddUserResponse"></a>

### AddUserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customer | [CustomerObj](#customer.CustomerObj) |  |  |






<a name="customer.Address"></a>

### Address



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| zip | [string](#string) |  |  |
| location | [string](#string) |  |  |
| address | [string](#string) |  |  |






<a name="customer.CreateNewRequest"></a>

### CreateNewRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| email | [string](#string) |  |  |
| phone | [string](#string) |  |  |
| tax_number | [string](#string) |  |  |
| zip | [string](#string) |  |  |
| location | [string](#string) |  |  |
| address | [string](#string) |  |  |
| created_by | [string](#string) |  |  |






<a name="customer.CreateNewResponse"></a>

### CreateNewResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customer | [CustomerObj](#customer.CustomerObj) |  |  |






<a name="customer.CustomerObj"></a>

### CustomerObj



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| email | [string](#string) |  |  |
| phone | [string](#string) |  |  |
| tax_number | [string](#string) |  |  |
| address | [Address](#customer.Address) |  |  |
| has_user | [bool](#bool) |  |  |
| users | [string](#string) | repeated |  |
| date_created | [string](#string) |  |  |
| created_by | [string](#string) |  | next = 11;; |






<a name="customer.CustomerUpdateObj"></a>

### CustomerUpdateObj



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| address | [Address](#customer.Address) |  |  |
| email | [string](#string) |  |  |
| phone | [string](#string) |  |  |
| tax_number | [string](#string) |  |  |






<a name="customer.GetAllResponse"></a>

### GetAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customers | [CustomerObj](#customer.CustomerObj) | repeated |  |






<a name="customer.GetByIdRequest"></a>

### GetByIdRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customer_id | [string](#string) |  |  |






<a name="customer.GetByIdResponse"></a>

### GetByIdResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customer | [CustomerObj](#customer.CustomerObj) |  |  |






<a name="customer.RemoveUserRequest"></a>

### RemoveUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customer_id | [string](#string) |  |  |
| user_id | [string](#string) |  |  |






<a name="customer.RemoveUserResponse"></a>

### RemoveUserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customer | [CustomerObj](#customer.CustomerObj) |  |  |






<a name="customer.UpdateByIdRequest"></a>

### UpdateByIdRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customer_id | [string](#string) |  |  |
| customer | [CustomerUpdateObj](#customer.CustomerUpdateObj) |  |  |






<a name="customer.UpdateByIdResponse"></a>

### UpdateByIdResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| customer | [CustomerObj](#customer.CustomerObj) |  |  |





 

 

 


<a name="customer.Customer"></a>

### Customer


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateNew | [CreateNewRequest](#customer.CreateNewRequest) | [CreateNewResponse](#customer.CreateNewResponse) |  |
| GetAll | [.google.protobuf.Empty](#google.protobuf.Empty) | [GetAllResponse](#customer.GetAllResponse) |  |
| GetById | [GetByIdRequest](#customer.GetByIdRequest) | [GetByIdResponse](#customer.GetByIdResponse) |  |
| UpdateById | [UpdateByIdRequest](#customer.UpdateByIdRequest) | [UpdateByIdResponse](#customer.UpdateByIdResponse) |  |
| AddUser | [AddUserRequest](#customer.AddUserRequest) | [AddUserResponse](#customer.AddUserResponse) |  |
| RemoveUser | [RemoveUserRequest](#customer.RemoveUserRequest) | [RemoveUserResponse](#customer.RemoveUserResponse) |  |

 



<a name="email.proto"></a>
<p align="right"><a href="#top">Top</a></p>

## email.proto



<a name="email.EmailRequest"></a>

### EmailRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| to | [string](#string) |  |  |
| subject | [string](#string) |  |  |
| body | [string](#string) |  |  |





 

 

 


<a name="email.Email"></a>

### Email


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SendEmail | [EmailRequest](#email.EmailRequest) | [.google.protobuf.Empty](#google.protobuf.Empty) |  |

 



<a name="prelude.proto"></a>
<p align="right"><a href="#top">Top</a></p>

## prelude.proto



<a name="prelude.DateTime"></a>

### DateTime



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| rfc_3399 | [string](#string) |  |  |






<a name="prelude.FloatValue"></a>

### FloatValue



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [float](#float) |  |  |






<a name="prelude.UserId"></a>

### UserId



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| userid | [string](#string) |  |  |





 

 

 

 



<a name="product.proto"></a>
<p align="right"><a href="#top">Top</a></p>

## product.proto



<a name="product.CreateNewRequest"></a>

### CreateNewRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| quantity | [string](#string) |  |  |
| unit | [string](#string) |  |  |
| created_by | [string](#string) |  |  |






<a name="product.CreateNewResponse"></a>

### CreateNewResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| product | [ProductObj](#product.ProductObj) |  |  |






<a name="product.GetAllResponse"></a>

### GetAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| products | [ProductObj](#product.ProductObj) | repeated |  |






<a name="product.GetByIdRequest"></a>

### GetByIdRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sku | [string](#string) |  |  |






<a name="product.GetByIdResponse"></a>

### GetByIdResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| product | [ProductObj](#product.ProductObj) |  |  |






<a name="product.IsSkuRequest"></a>

### IsSkuRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sku | [string](#string) |  |  |






<a name="product.IsSkuResponse"></a>

### IsSkuResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sku_exist | [bool](#bool) |  |  |






<a name="product.ProductObj"></a>

### ProductObj



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sku | [string](#string) |  |  |
| name | [string](#string) |  |  |
| quantity | [string](#string) |  |  |
| unit | [string](#string) |  | impl Into&lt;String&gt; for Unit {} &amp; from_str() |
| created_by | [string](#string) |  | Maybe UserId? And impl From&lt;..&gt; for it? |
| created_at | [string](#string) |  |  |






<a name="product.ProductUpdateObj"></a>

### ProductUpdateObj



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sku | [string](#string) |  |  |
| name | [string](#string) |  |  |
| quantity | [string](#string) |  |  |
| unit | [string](#string) |  |  |






<a name="product.SKU"></a>

### SKU



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sku | [string](#string) |  |  |






<a name="product.UpdateByIdRequest"></a>

### UpdateByIdRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| product | [ProductUpdateObj](#product.ProductUpdateObj) |  |  |






<a name="product.UpdateByIdResponse"></a>

### UpdateByIdResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| product | [ProductObj](#product.ProductObj) |  | Product Object After an update call, it will return the updated object for validation |





 

 

 


<a name="product.Product"></a>

### Product
Product Service
All the methods related to manage Product

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateNew | [CreateNewRequest](#product.CreateNewRequest) | [CreateNewResponse](#product.CreateNewResponse) |  |
| GetAll | [.google.protobuf.Empty](#google.protobuf.Empty) | [GetAllResponse](#product.GetAllResponse) |  |
| GetById | [GetByIdRequest](#product.GetByIdRequest) | [GetByIdResponse](#product.GetByIdResponse) |  |
| UpdateById | [UpdateByIdRequest](#product.UpdateByIdRequest) | [UpdateByIdResponse](#product.UpdateByIdResponse) |  |
| isSku | [IsSkuRequest](#product.IsSkuRequest) | [IsSkuResponse](#product.IsSkuResponse) |  |

 



<a name="source.proto"></a>
<p align="right"><a href="#top">Top</a></p>

## source.proto



<a name="source.PriceHistory"></a>

### PriceHistory



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sku | [string](#string) |  |  |
| items | [PriceHistoryItem](#source.PriceHistoryItem) | repeated | next: |






<a name="source.PriceHistoryItem"></a>

### PriceHistoryItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| date | [string](#string) |  |  |
| created_by | [string](#string) |  |  |
| net_wholesale_price | [uint32](#uint32) |  |  |
| comment | [string](#string) |  |  |
| kind | [string](#string) |  | next: |






<a name="source.Todo"></a>

### Todo






 

 

 


<a name="source.Source"></a>

### Source


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| NewSource | [Todo](#source.Todo) | [Todo](#source.Todo) | Create new source |
| GetSourceById | [Todo](#source.Todo) | [Todo](#source.Todo) | Get source by id |
| GetAllSources | [Todo](#source.Todo) | [Todo](#source.Todo) | Get list of sources |
| UpdateSourceById | [Todo](#source.Todo) | [Todo](#source.Todo) stream | Update source by id |
| AddNewSkuToSource | [Todo](#source.Todo) | [Todo](#source.Todo) | Add new sku to source |
| GetPriceBySku | [Todo](#source.Todo) | [Todo](#source.Todo) | Get price by SKU |
| GetPriceHistoryBySku | [Todo](#source.Todo) | [Todo](#source.Todo) | Get price history by SKU |

 



<a name="upl.proto"></a>
<p align="right"><a href="#top">Top</a></p>

## upl.proto



<a name="upl.CreateNewRequest"></a>

### CreateNewRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint32](#uint32) |  |  |
| sku | [string](#string) |  | .. |
| created_by | [string](#string) |  |  |
| date_created | [string](#string) |  |  |






<a name="upl.CreateNewResponse"></a>

### CreateNewResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| upl | [Upl](#upl.Upl) |  |  |






<a name="upl.GetUplRequest"></a>

### GetUplRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| upl_id | [uint32](#uint32) |  |  |






<a name="upl.GetUplResponse"></a>

### GetUplResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| upl | [Upl](#upl.Upl) |  |  |






<a name="upl.MoveRequest"></a>

### MoveRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| from_type | [string](#string) |  | e.g. stock |
| from_id | [string](#string) |  | e.g. 324324 |
| to_type | [string](#string) |  | e.g. cart |
| to_id | [string](#string) |  | e.g. 4367 |






<a name="upl.MoveResponse"></a>

### MoveResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todo | [string](#string) |  |  |
| todo2 | [string](#string) |  |  |






<a name="upl.Upl"></a>

### Upl



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [uint64](#uint64) |  |  |
| sku | [string](#string) |  |  |
| location_history | [UplHistoryItem](#upl.UplHistoryItem) | repeated | ... |
| created_by | [string](#string) |  |  |
| date_created | [string](#string) |  |  |






<a name="upl.UplByPlaceRequest"></a>

### UplByPlaceRequest







<a name="upl.UplHistoryItem"></a>

### UplHistoryItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| location | [string](#string) |  |  |
| id | [string](#string) |  |  |
| date_created | [string](#string) |  |  |
| created_by | [string](#string) |  |  |





 

 

 


<a name="upl.UplStore"></a>

### UplStore


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateNew | [CreateNewRequest](#upl.CreateNewRequest) | [CreateNewResponse](#upl.CreateNewResponse) |  |
| GetById | [GetUplRequest](#upl.GetUplRequest) | [GetUplResponse](#upl.GetUplResponse) |  |
| Move | [MoveRequest](#upl.MoveRequest) | [MoveResponse](#upl.MoveResponse) |  |
| GetByPlace | [UplByPlaceRequest](#upl.UplByPlaceRequest) | [Upl](#upl.Upl) stream |  |

 



<a name="user.proto"></a>
<p align="right"><a href="#top">Top</a></p>

## user.proto



<a name="user.AllUserRequest"></a>

### AllUserRequest







<a name="user.CreateNewRequest"></a>

### CreateNewRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| email | [string](#string) |  |  |
| name | [string](#string) |  |  |
| phone | [string](#string) |  |  |
| created_by | [string](#string) |  |  |






<a name="user.CreateNewResponse"></a>

### CreateNewResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [UserObj](#user.UserObj) |  |  |






<a name="user.GetAllResponse"></a>

### GetAllResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| users | [UserObj](#user.UserObj) | repeated |  |






<a name="user.GetByIdRequest"></a>

### GetByIdRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| userid | [string](#string) |  |  |






<a name="user.GetByIdResponse"></a>

### GetByIdResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [UserObj](#user.UserObj) |  |  |






<a name="user.IsUserRequest"></a>

### IsUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| userid | [string](#string) |  |  |






<a name="user.IsUserResponse"></a>

### IsUserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_exist | [bool](#bool) |  |  |






<a name="user.LoginRequest"></a>

### LoginRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| username | [string](#string) |  |  |
| password | [string](#string) |  |  |






<a name="user.LoginResponse"></a>

### LoginResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_valid | [bool](#bool) |  |  |
| user | [UserObj](#user.UserObj) |  |  |






<a name="user.NewPasswordRequest"></a>

### NewPasswordRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| userid | [string](#string) |  |  |
| new_password | [string](#string) |  |  |






<a name="user.NewPasswordResponse"></a>

### NewPasswordResponse







<a name="user.ResetPasswordRequest"></a>

### ResetPasswordRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| email | [string](#string) |  |  |






<a name="user.ResetPasswordResponse"></a>

### ResetPasswordResponse







<a name="user.UpdateByIdRequest"></a>

### UpdateByIdRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [UserObj](#user.UserObj) |  |  |






<a name="user.UpdateByIdResponse"></a>

### UpdateByIdResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [UserObj](#user.UserObj) |  |  |






<a name="user.UserObj"></a>

### UserObj



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| email | [string](#string) |  |  |
| phone | [string](#string) |  |  |
| customers | [string](#string) | repeated | This will be Vec&lt;String&gt; |
| created_by | [string](#string) |  |  |
| created_at | [string](#string) |  | maybe datetime type? |





 

 

 


<a name="user.User"></a>

### User


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateNew | [CreateNewRequest](#user.CreateNewRequest) | [CreateNewResponse](#user.CreateNewResponse) | CreateNewUser CreateNewRequest -&gt; CreateNewResponse |
| GetAll | [.google.protobuf.Empty](#google.protobuf.Empty) | [UserObj](#user.UserObj) stream | Get all users AllUserRequest -&gt; GerAllResponse |
| GetById | [GetByIdRequest](#user.GetByIdRequest) | [GetByIdResponse](#user.GetByIdResponse) | Get User By Id GetByIdRequest -&gt; UserByIdResponse |
| UpdateById | [UpdateByIdRequest](#user.UpdateByIdRequest) | [UpdateByIdResponse](#user.UpdateByIdResponse) | Update User By ID UpdateByIdRequest -&gt; UpdateUserReply |
| IsUser | [IsUserRequest](#user.IsUserRequest) | [IsUserResponse](#user.IsUserResponse) | rpc CheckUserExist(google.protobuf.Empty) returns (google.protobuf.Empty); |
| ResetPassword | [ResetPasswordRequest](#user.ResetPasswordRequest) | [ResetPasswordResponse](#user.ResetPasswordResponse) | ResetPassword -&gt; Email::send(!) |
| SetNewPassword | [NewPasswordRequest](#user.NewPasswordRequest) | [NewPasswordResponse](#user.NewPasswordResponse) | Set new password |
| ValidateLogin | [LoginRequest](#user.LoginRequest) | [LoginResponse](#user.LoginResponse) | Validate login request |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

