import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  scenarios: {
    contacts: {
      executor: 'ramping-arrival-rate',
      preAllocatedVUs: 50,
      timeUnit: '1s',
      startRate: 50,
      stages: [
        { target: 200, duration: '10s' }, // linearly go from 50 iters/s to 200 iters/s for 30s
        { target: 500, duration: '0' }, // instantly jump to 500 iters/s
        // { target: 500, duration: '10m' }, // continue with 500 iters/s for 10 minutes
      ],
    },
  },
};

export default function () {
  let res = http.get("http://localhost:8003/healt/status");
  // Validate response status
  check(res, { "status was 200": (r) => r.status == 200 });
//   sleep(1);
}