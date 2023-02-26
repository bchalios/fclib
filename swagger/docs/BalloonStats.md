# BalloonStats

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_pages** | **i32** | Target number of pages the device aims to hold. | [default to null]
**actual_pages** | **i32** | Actual number of pages the device is holding. | [default to null]
**target_mib** | **i32** | Target amount of memory (in MiB) the device aims to hold. | [default to null]
**actual_mib** | **i32** | Actual amount of memory (in MiB) the device is holding. | [default to null]
**swap_in** | **i64** | The amount of memory that has been swapped in (in bytes). | [optional] [default to null]
**swap_out** | **i64** | The amount of memory that has been swapped out to disk (in bytes). | [optional] [default to null]
**major_faults** | **i64** | The number of major page faults that have occurred. | [optional] [default to null]
**minor_faults** | **i64** | The number of minor page faults that have occurred. | [optional] [default to null]
**free_memory** | **i64** | The amount of memory not being used for any purpose (in bytes). | [optional] [default to null]
**total_memory** | **i64** | The total amount of memory available (in bytes). | [optional] [default to null]
**available_memory** | **i64** | An estimate of how much memory is available (in bytes) for starting new applications, without pushing the system to swap. | [optional] [default to null]
**disk_caches** | **i64** | The amount of memory, in bytes, that can be quickly reclaimed without additional I/O. Typically these pages are used for caching files from disk. | [optional] [default to null]
**hugetlb_allocations** | **i64** | The number of successful hugetlb page allocations in the guest. | [optional] [default to null]
**hugetlb_failures** | **i64** | The number of failed hugetlb page allocations in the guest. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


