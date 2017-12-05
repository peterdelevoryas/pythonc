# Contributed Tests

Contribute your test pairs to this repository using the following protocol.

The repository is organized into the sub-directory for the appropriate language subset:

```
.
├── P0_tests
├── P1_tests
├── P2_tests
├── P3_tests
└── README.md
```

A test pair is a Python `.py` file and a input `.in` file for that test program.

To submit a test pair, please do the following:

1. Prefix your test-pair files with your team name from your lab GitHub repository. For example, if your lab GitHub repository is `pyyc-gracehopper`, then you would submit a test pair called, for example, `P0_tests/gracehopper-superhardp0.py` with the P0 program and `P0_tests/gracehopper-superhardp0.in` with a corresponding input.

2. Push your test-pair files into the `submit` branch. Everyone in the course should have push access to this branch. The default branch for the repository has been configured to be the `submit` branch, so you should not have to change branches after cloning. For example,
```bash
~/pyyc-tests-contrib (submit)$ git add P0_tests/gracehopper-superhardp0.py P0_tests/gracehopper-superhardp0.in
~/pyyc-tests-contrib (submit)$ git commit -m "Add the gracehopper-superhardp0 test pair"
~/pyyc-tests-contrib (submit)$ git push
```

3. Copy a link to your commit (e.g., https://github.com/csci4555-f17/pyyc-tests-contrib/pull/1/commits/be600d74ce76b168a6237c2264aa74657bcc561e), which you can find from the open [Pull Request #1](https://github.com/csci4555-f17/pyyc-tests-contrib/pull/1).
