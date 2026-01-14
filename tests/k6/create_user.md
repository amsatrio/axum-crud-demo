# service - mariadb
    TOTAL RESULTS 

    checks_total.......: 19397   114.086724/s
    checks_succeeded...: 100.00% 19397 out of 19397
    checks_failed......: 0.00%   0 out of 19397

    ✓ is status 201 or 200

    HTTP
    http_req_duration..............: avg=975.08ms min=2.74ms   med=801.01ms max=4.2s p(90)=1.95s p(95)=2.36s
      { expected_response:true }...: avg=975.08ms min=2.74ms   med=801.01ms max=4.2s p(90)=1.95s p(95)=2.36s
    http_req_failed................: 0.00%  0 out of 19397
    http_reqs......................: 19397  114.086724/s

    EXECUTION
    dropped_iterations.............: 21902  128.820304/s
    iteration_duration.............: avg=1.07s    min=103.24ms med=902.23ms max=4.3s p(90)=2.05s p(95)=2.46s
    iterations.....................: 19397  114.086724/s
    vus............................: 1      min=1          max=500
    vus_max........................: 500    min=100        max=500

    NETWORK
    data_received..................: 6.4 MB 37 kB/s
    data_sent......................: 3.3 MB 20 kB/s

# service - redis - service - mariadb