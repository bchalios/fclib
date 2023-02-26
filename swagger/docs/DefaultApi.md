# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snapshot**](DefaultApi.md#create_snapshot) | **Put** /snapshot/create | Creates a full or diff snapshot. Post-boot only.
[**create_sync_action**](DefaultApi.md#create_sync_action) | **Put** /actions | Creates a synchronous action.
[**describe_balloon_config**](DefaultApi.md#describe_balloon_config) | **Get** /balloon | Returns the current balloon device configuration.
[**describe_balloon_stats**](DefaultApi.md#describe_balloon_stats) | **Get** /balloon/statistics | Returns the latest balloon device statistics, only if enabled pre-boot.
[**describe_instance**](DefaultApi.md#describe_instance) | **Get** / | Returns general information about an instance.
[**get_export_vm_config**](DefaultApi.md#get_export_vm_config) | **Get** /vm/config | Gets the full VM configuration.
[**get_firecracker_version**](DefaultApi.md#get_firecracker_version) | **Get** /version | Gets the Firecracker version.
[**get_machine_configuration**](DefaultApi.md#get_machine_configuration) | **Get** /machine-config | Gets the machine configuration of the VM.
[**get_mmds**](DefaultApi.md#get_mmds) | **Get** /mmds | Get the MMDS data store.
[**load_snapshot**](DefaultApi.md#load_snapshot) | **Put** /snapshot/load | Loads a snapshot. Pre-boot only.
[**patch_balloon**](DefaultApi.md#patch_balloon) | **Patch** /balloon | Updates a balloon device.
[**patch_balloon_stats_interval**](DefaultApi.md#patch_balloon_stats_interval) | **Patch** /balloon/statistics | Updates a balloon device statistics polling interval.
[**patch_guest_drive_by_id**](DefaultApi.md#patch_guest_drive_by_id) | **Patch** /drives/{drive_id} | Updates the properties of a drive. Post-boot only.
[**patch_guest_network_interface_by_id**](DefaultApi.md#patch_guest_network_interface_by_id) | **Patch** /network-interfaces/{iface_id} | Updates the rate limiters applied to a network interface. Post-boot only.
[**patch_machine_configuration**](DefaultApi.md#patch_machine_configuration) | **Patch** /machine-config | Partially updates the Machine Configuration of the VM. Pre-boot only.
[**patch_mmds**](DefaultApi.md#patch_mmds) | **Patch** /mmds | Updates the MMDS data store.
[**patch_vm**](DefaultApi.md#patch_vm) | **Patch** /vm | Updates the microVM state.
[**put_balloon**](DefaultApi.md#put_balloon) | **Put** /balloon | Creates or updates a balloon device.
[**put_guest_boot_source**](DefaultApi.md#put_guest_boot_source) | **Put** /boot-source | Creates or updates the boot source. Pre-boot only.
[**put_guest_drive_by_id**](DefaultApi.md#put_guest_drive_by_id) | **Put** /drives/{drive_id} | Creates or updates a drive. Pre-boot only.
[**put_guest_network_interface_by_id**](DefaultApi.md#put_guest_network_interface_by_id) | **Put** /network-interfaces/{iface_id} | Creates a network interface. Pre-boot only.
[**put_guest_vsock**](DefaultApi.md#put_guest_vsock) | **Put** /vsock | Creates/updates a vsock device. Pre-boot only.
[**put_logger**](DefaultApi.md#put_logger) | **Put** /logger | Initializes the logger by specifying a named pipe or a file for the logs output.
[**put_machine_configuration**](DefaultApi.md#put_machine_configuration) | **Put** /machine-config | Updates the Machine Configuration of the VM. Pre-boot only.
[**put_metrics**](DefaultApi.md#put_metrics) | **Put** /metrics | Initializes the metrics system by specifying a named pipe or a file for the metrics output.
[**put_mmds**](DefaultApi.md#put_mmds) | **Put** /mmds | Creates a MMDS (Microvm Metadata Service) data store.
[**put_mmds_config**](DefaultApi.md#put_mmds_config) | **Put** /mmds/config | Set MMDS configuration. Pre-boot only.


# **create_snapshot**
> create_snapshot(body)
Creates a full or diff snapshot. Post-boot only.

Creates a snapshot of the microVM state. The microVM should be in the `Paused` state.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**SnapshotCreateParams**](SnapshotCreateParams.md)| The configuration used for creating a snaphot. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_sync_action**
> create_sync_action(info)
Creates a synchronous action.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **info** | [**InstanceActionInfo**](InstanceActionInfo.md)|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **describe_balloon_config**
> ::models::Balloon describe_balloon_config()
Returns the current balloon device configuration.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::Balloon**](Balloon.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **describe_balloon_stats**
> ::models::BalloonStats describe_balloon_stats()
Returns the latest balloon device statistics, only if enabled pre-boot.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::BalloonStats**](BalloonStats.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **describe_instance**
> ::models::InstanceInfo describe_instance()
Returns general information about an instance.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InstanceInfo**](InstanceInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_export_vm_config**
> ::models::FullVmConfiguration get_export_vm_config()
Gets the full VM configuration.

Gets configuration for all VM resources. If the VM is restored from a snapshot, the boot-source, machine-config.smt and machine-config.cpu_template will be empty.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::FullVmConfiguration**](FullVmConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_firecracker_version**
> ::models::FirecrackerVersion get_firecracker_version()
Gets the Firecracker version.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::FirecrackerVersion**](FirecrackerVersion.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_machine_configuration**
> ::models::MachineConfiguration get_machine_configuration()
Gets the machine configuration of the VM.

Gets the machine configuration of the VM. When called before the PUT operation, it will return the default values for the vCPU count (=1), memory size (=128 MiB). By default SMT is disabled and there is no CPU Template.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::MachineConfiguration**](MachineConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mmds**
> Value get_mmds()
Get the MMDS data store.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **load_snapshot**
> load_snapshot(body)
Loads a snapshot. Pre-boot only.

Loads the microVM state from a snapshot. Only accepted on a fresh Firecracker process (before configuring any resource other than the Logger and Metrics).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**SnapshotLoadParams**](SnapshotLoadParams.md)| The configuration used for loading a snaphot. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_balloon**
> patch_balloon(body)
Updates a balloon device.

Updates an existing balloon device, before or after machine startup. Will fail if update is not possible.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**BalloonUpdate**](BalloonUpdate.md)| Balloon properties | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_balloon_stats_interval**
> patch_balloon_stats_interval(body)
Updates a balloon device statistics polling interval.

Updates an existing balloon device statistics interval, before or after machine startup. Will fail if update is not possible.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**BalloonStatsUpdate**](BalloonStatsUpdate.md)| Balloon properties | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_guest_drive_by_id**
> patch_guest_drive_by_id(drive_id, body)
Updates the properties of a drive. Post-boot only.

Updates the properties of the drive with the ID specified by drive_id path parameter. Will fail if update is not possible.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **drive_id** | **String**| The id of the guest drive | 
  **body** | [**PartialDrive**](PartialDrive.md)| Guest drive properties | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_guest_network_interface_by_id**
> patch_guest_network_interface_by_id(iface_id, body)
Updates the rate limiters applied to a network interface. Post-boot only.

Updates the rate limiters applied to a network interface.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **iface_id** | **String**| The id of the guest network interface | 
  **body** | [**PartialNetworkInterface**](PartialNetworkInterface.md)| A subset of the guest network interface properties | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_machine_configuration**
> patch_machine_configuration(optional)
Partially updates the Machine Configuration of the VM. Pre-boot only.

Partially updates the Virtual Machine Configuration with the specified input. If any of the parameters has an incorrect value, the whole update fails.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**MachineConfiguration**](MachineConfiguration.md)| A subset of Machine Configuration Parameters | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_mmds**
> patch_mmds(optional)
Updates the MMDS data store.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**MmdsContentsObject**](MmdsContentsObject.md)| The MMDS data store patch JSON. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_vm**
> patch_vm(body)
Updates the microVM state.

Sets the desired state (Paused or Resumed) for the microVM.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Vm**](Vm.md)| The microVM state | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_balloon**
> put_balloon(body)
Creates or updates a balloon device.

Creates a new balloon device if one does not already exist, otherwise updates it, before machine startup. This will fail after machine startup. Will fail if update is not possible.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Balloon**](Balloon.md)| Balloon properties | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_guest_boot_source**
> put_guest_boot_source(body)
Creates or updates the boot source. Pre-boot only.

Creates new boot source if one does not already exist, otherwise updates it. Will fail if update is not possible.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**BootSource**](BootSource.md)| Guest boot source properties | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_guest_drive_by_id**
> put_guest_drive_by_id(drive_id, body)
Creates or updates a drive. Pre-boot only.

Creates new drive with ID specified by drive_id path parameter. If a drive with the specified ID already exists, updates its state based on new input. Will fail if update is not possible.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **drive_id** | **String**| The id of the guest drive | 
  **body** | [**Drive**](Drive.md)| Guest drive properties | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_guest_network_interface_by_id**
> put_guest_network_interface_by_id(iface_id, body)
Creates a network interface. Pre-boot only.

Creates new network interface with ID specified by iface_id path parameter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **iface_id** | **String**| The id of the guest network interface | 
  **body** | [**NetworkInterface**](NetworkInterface.md)| Guest network interface properties | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_guest_vsock**
> put_guest_vsock(body)
Creates/updates a vsock device. Pre-boot only.

The first call creates the device with the configuration specified in body. Subsequent calls will update the device configuration. May fail if update is not possible.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Vsock**](Vsock.md)| Guest vsock properties | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_logger**
> put_logger(body)
Initializes the logger by specifying a named pipe or a file for the logs output.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Logger**](Logger.md)| Logging system description | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_machine_configuration**
> put_machine_configuration(optional)
Updates the Machine Configuration of the VM. Pre-boot only.

Updates the Virtual Machine Configuration with the specified input. Firecracker starts with default values for vCPU count (=1) and memory size (=128 MiB). The vCPU count is restricted to the [1, 32] range. With SMT enabled, the vCPU count is required to be either 1 or an even number in the range. otherwise there are no restrictions regarding the vCPU count. If any of the parameters has an incorrect value, the whole update fails. All parameters that are optional and are not specified are set to their default values (smt = false, track_dirty_pages = false, cpu_template = None).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**MachineConfiguration**](MachineConfiguration.md)| Machine Configuration Parameters | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_metrics**
> put_metrics(body)
Initializes the metrics system by specifying a named pipe or a file for the metrics output.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**Metrics**](Metrics.md)| Metrics system description | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_mmds**
> put_mmds(optional)
Creates a MMDS (Microvm Metadata Service) data store.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**MmdsContentsObject**](MmdsContentsObject.md)| The MMDS data store as JSON. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_mmds_config**
> put_mmds_config(body)
Set MMDS configuration. Pre-boot only.

Configures MMDS version, IPv4 address used by the MMDS network stack and interfaces that allow MMDS requests.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**MmdsConfig**](MmdsConfig.md)| The MMDS configuration as JSON. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

