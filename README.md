# split-test

split-test splits tests into multiple groups based on timing data to run tests in parallel.

## Installation

Download binary from GitHub releases page:

https://github.com/mtsmfm/split-test/releases

## Usage

split-test command outputs test groups to stdout depends on its executing time.

Let's say we have three test files `spec/a_spec.rb`, `spec/b_spec.rb`, `spec/c_spec.rb` and the following test report `report/report.xml`.

```xml
<testsuites>
<testsuite classname="all" name="all" time="18.0">
<testcase classname="a" name="a" file="./spec/a_spec.rb" time="5.0"></testcase>
<testcase classname="b" name="b" file="./spec/b_spec.rb" time="10.0"></testcase>
<testcase classname="c" name="c" file="./spec/c_spec.rb" time="3.0"></testcase>
</testsuite>
</testsuites>
```

If you run them in series, it'll take 18 seconds.

You can use `split-test` to split them into two groups and run in parallel.

For node 0:

```
$ split-test --junit-xml-report-dir report --node-index 0 --node-total 2 --tests-glob 'spec/**/*_spec.rb'
```

You'll get the following result on stdout:

```
/path/to/spec/a_spec.rb
/path/to/spec/c_spec.rb
```

For node 1:

```
$ split-test --junit-xml-report-dir . --node-index 1 --node-total 2 --tests-glob 'spec/**/*_spec.rb'
```

You'll get:

```
/path/to/spec/b_spec.rb
```

Please be sure to increment `--node-index` arg.

You can use `--debug` option to make sure how it's grouped:

```
$ split-test --junit-xml-report-dir . --node-index 1 --node-total 2 --tests-glob 'spec/**/*_spec.rb' --debug
[2021-01-09T02:55:04Z DEBUG split_test] {"/path/to/spec/b_spec.rb": 10.0, "/path/to/spec/c_spec.rb": 3.0, "/path/to/spec/a_spec.rb": 5.0}
[2021-01-09T02:55:04Z DEBUG split_test] node 0: recorded_total_time: 8
[2021-01-09T02:55:04Z DEBUG split_test] /path/to/spec/a_spec.rb
[2021-01-09T02:55:04Z DEBUG split_test] /path/to/spec/c_spec.rb
[2021-01-09T02:55:04Z DEBUG split_test] node 1: recorded_total_time: 10
[2021-01-09T02:55:04Z DEBUG split_test] /path/to/spec/b_spec.rb
/path/to/spec/b_spec.rb
```

Pass the result to test command to run grouped tests:

```
$ rspec $(split-test --junit-xml-report-dir report --node-index 0 --node-total 2 --tests-glob 'spec/**/*_spec.rb' --debug)
```

### GitHub Actions

```yaml
on: push

jobs:
  # Download test-report and save as test-report-tmp to use the exactly same test report across parallel jobs.
  download-test-report:
    runs-on: ubuntu-latest
    steps:
      # Use dawidd6/action-download-artifact to download JUnit Format XML test report from another branch
      # https://github.com/actions/download-artifact/issues/3
      - uses: dawidd6/action-download-artifact@v2
        with:
          branch: main
          name: test-report
          workflow: ci.yml
          path: report
        # Use continue-on-error to run tests even if test-report is not uploaded
        continue-on-error: true
      - uses: actions/upload-artifact@v2
        with:
          name: test-report-tmp
          path: report

  test:
    needs: download-test-report
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node_index: [0, 1, 2]
    steps:
      - uses: actions/checkout@v2
      - uses: ruby/setup-ruby@v1
        with:
          bundler-cache: true
          ruby-version: 3.0.0
      - uses: actions/download-artifact@v2
        with:
          name: test-report-tmp
          path: report-tmp
        # Use continue-on-error to run tests even if test-report is not uploaded
        continue-on-error: true
      - run: |
          curl -L --out split-test https://github.com/mtsmfm/split-test/releases/download/v1.0.0/split-test-x86_64-unknown-linux-gnu
          chmod +x split-test
      - run: bin/rspec --format progress --format RspecJunitFormatter --out report/rspec-${{ matrix.node_index }}.xml $(./split-test --junit-xml-report-dir report-tmp --node-index ${{ matrix.node_index }} --node-total 3 --tests-glob 'spec/**/*_spec.rb' --debug)
      - uses: actions/upload-artifact@v2
        with:
          name: test-report
          path: report
          if-no-files-found: error
        # Upload test-report on main branch only to avoid conflicting test report
        if: github.ref == 'refs/heads/main'
```

You can find working example on https://github.com/mtsmfm/split-test-example

## Note

split-test is inspired by [`circleci tests split` command](https://circleci.com/docs/2.0/parallelism-faster-jobs/).

split-test assumes test report has additional attribute `file` or `filepath`.

```xml
<testsuites>
<testsuite>
<testcase classname="spec.0_spec" name="0 is expected to eq 0" file="./spec/0_spec.rb" time="0.000373"></testcase>
</testsuite>
</testsuites>
```

To be exact it isn't JUnit standard.
