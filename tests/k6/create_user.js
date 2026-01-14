import http from 'k6/http';
import { check, sleep } from 'k6';
import { randomString } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

export const options = {
  scenarios: {
    spike_test: {
      executor: 'ramping-arrival-rate',
      preAllocatedVUs: 100, // Increased to handle the spike
      maxVUs: 10000,          // Upper limit of VUs k6 can scale to
      timeUnit: '1s',
      startRate: 10,
      stages: [
        { target: 10, duration: '30' },   // 1. Warm up at low traffic
        { target: 1000, duration: '10s' }, // 2. SPIKE: Jump to 1000 iterations/sec
        { target: 1000, duration: '30s' }, // 3. Hold the spike
        { target: 10, duration: '10s' },  // 4. Scale down quickly
        { target: 10, duration: '1m' },   // 5. Recovery period
      ],
    },
  },
};

export default function () {
  const url = 'http://localhost:8003/quiz/create_new_user';
  
  // Generate a random username (e.g., user_abc123)
  const payload = JSON.stringify({
    username: `user_${randomString(8)}`,
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  let res = http.post(url, payload, params);

  check(res, {
    'is status 201 or 200': (r) => r.status === 201 || r.status === 200,
  });

  // Short sleep to prevent tight-looping within the arrival rate
//   sleep(0.1);
}