# SnapshotLoadParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_diff_snapshots** | **bool** | Enable support for incremental (diff) snapshots by tracking dirty guest pages. | [optional] [default to null]
**mem_file_path** | **String** | Path to the file that contains the guest memory to be loaded. This parameter has been deprecated and is only allowed if &#x60;mem_backend&#x60; is not present. | [optional] [default to null]
**mem_backend** | [***::models::MemoryBackend**](MemoryBackend.md) | Configuration for the backend that handles memory load. If this field is specified, &#x60;mem_file_path&#x60; is forbidden. Either &#x60;mem_backend&#x60; or &#x60;mem_file_path&#x60; must be present at a time. | [optional] [default to null]
**snapshot_path** | **String** | Path to the file that contains the microVM state to be loaded. | [default to null]
**resume_vm** | **bool** | When set to true, the vm is also resumed if the snapshot load is successful. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


