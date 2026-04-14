Nos presentaron la siguiente pregunta o conclusion para analizar:Ya tenemos el proyecto operativo y algunas devoluciones que tuvimos fueron las siguientes:carlos-acuna@fedora:~/Projects/concurrency$ ab -n 500 -c 50 http://localhost:3030/pi/1000000

This is ApacheBench, Version 2.3 <$Revision: 1923142 $>

Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/

Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)

Completed 100 requests

Completed 200 requests

Completed 300 requests

Completed 400 requests

Completed 500 requests

Finished 500 requests

Server Software:

Server Hostname: localhost

Server Port: 3030

Document Path: /pi/1000000

Document Length: 80 bytes

Concurrency Level: 50

Time taken for tests: 0.750 seconds

Complete requests: 500

Failed requests: 0

Total transferred: 59500 bytes

HTML transferred: 40000 bytes

Requests per second: 666.78 \[#/sec\] (mean)

Time per request: 74.987 \[ms\] (mean)

Time per request: 1.500 \[ms\] (mean, across all concurrent requests)

Transfer rate: 77.49 \[Kbytes/sec\] received

Connection Times (ms)

min mean\[+/-sd\] median max

Connect: 0 0 0.7 0 9

Processing: 6 73 54.7 57 301

Waiting: 5 72 54.7 57 301

Total: 6 73 54.9 58 301

Percentage of the requests served within a certain time (ms)

50% 58

66% 84

75% 107

80% 120

90% 145

95% 181

98% 226

99% 246

100% 301 (longest request)

Segun algunas cuestiones, nos consultaron:Bajo carga concurrente activa