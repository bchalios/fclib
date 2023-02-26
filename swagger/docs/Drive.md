# Drive

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**drive_id** | **String** |  | [default to null]
**cache_type** | **String** | Represents the caching strategy for the block device. | [optional] [default to null]
**is_read_only** | **bool** |  | [default to null]
**is_root_device** | **bool** |  | [default to null]
**partuuid** | **String** | Represents the unique id of the boot partition of this device. It is optional and it will be taken into account only if the is_root_device field is true. | [optional] [default to null]
**path_on_host** | **String** | Host level path for the guest drive | [default to null]
**rate_limiter** | [***::models::RateLimiter**](RateLimiter.md) |  | [optional] [default to null]
**io_engine** | **String** | Type of the IO engine used by the device. \&quot;Async\&quot; is supported on host kernels newer than 5.10.51. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


