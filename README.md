v0.1 USABLE

# Kubus - local kubernetes batch operations tool

> [!WARNING]
> The project is designed for my personal needs only, and may not be suitable for everybody. I do not offer any support for it either. It's completely Open Source, fork it and adapt it to your needs.

Kubus helps with massive deletions from cluster, based on name of instance, namespaces, instance type.
Default namespace is used if none specified.
## Quick Start

```console
$ kubus --fanl revify --delete --svc    
{_Service: ["revify-eureka-cip", "revify-eureka-lb"], _Pod: ["revify-eureka-589bff7f99-q6b2x"], _Deployment: ["revify-eureka"], _PersistentVolumeClaim: []}
Deleting svc with name revify-eureka-cip for namespace default
Deleting svc with name revify-eureka-lb for namespace default
$ kubus --fanl revify
{_Service: [], _Pod: ["revify-eureka-589bff7f99-q6b2x"], _Deployment: ["revify-eureka"], _PersistentVolumeClaim: []}
$ kubus --fanl revify --dpl 
{_Deployment: ["revify-eureka"]}
```

## Operations

For more information on how to manipulate run `kubus --help`.
Feel free to contribute.

