#Getting Started with Envoy
To Do
- Make a simple Rust based service that returns the hostname (Done)
- Get Envoy proxying to new Service (Done)
- Use envoy to route two vhosts using different hostnames and point to same rust service (Done)
- create SSL certificates for two hostnames and terminate on envoy ()
- create service discovery for envoy virtual hosts
- create secrets discovery for envoy virtual hosts TLS termination
- create script that deploys new vhost and cert to service discovery and test that envoy can route based on new hostname


It seems I can't do what I want yet:
In order to scale to thousands of domains, I want filter chain discovery service and that does not exist.
https://github.com/envoyproxy/envoy/issues/16174