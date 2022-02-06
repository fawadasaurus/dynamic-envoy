#Getting Started with Envoy
To Do
- Make a simple Rust based service that returns the hostname (replace HTTP Bin)
- Get Envoy proxying to new Service
- Use envoy to route two vhosts using different hostnames and point to same rust service
- create SSL certificates for two hostnames and terminate on envoy
- create service discovery for envoy virtual hosts
- create secrets discovery for envoy virtual hosts TLS termination
- create script that deploys new vhost and cert to service discovery and test that envoy can route based on new hostname
