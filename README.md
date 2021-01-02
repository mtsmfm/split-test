# split-test

split-test splits tests into multiple groups based on timing data to run tests in parallel.

## Installation

Download binary from GitHub releases page:

https://github.com/mtsmfm/split-test/releases

## Usage

```
$ split-test --junit-xml-report-dir reports --node-index 0 --node-total 3 --tests-glob 'spec/**/*_spec.rb' --debug
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
          path: reports
        # Use continue-on-error to run tests even if test-report is not uploaded
        continue-on-error: true
      - uses: actions/upload-artifact@v2
        with:
          name: test-report-tmp
          path: reports

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
          path: reports
        # Use continue-on-error to run tests even if test-report is not uploaded
        continue-on-error: true
      - run: |
          curl -L --out split-test https://github.com/mtsmfm/split-test/releases/download/v0.3.0/split-test-x86_64-unknown-linux-gnu
          chmod +x split-test
      - run: bin/rspec --format progress --format RspecJunitFormatter --out reports/rspec-${{ matrix.node_index }}.xml $(./split-test --junit-xml-report-dir reports --node-index ${{ matrix.node_index }} --node-total 3 --tests-glob 'spec/**/*_spec.rb' --debug)
      - uses: actions/upload-artifact@v2
        with:
          name: test-report
          path: reports
          if-no-files-found: error
        # Upload test-report on main branch only to avoid conflicting test report
        if: github.ref == 'refs/heads/main'
```

You can find working example on https://github.com/mtsmfm/split-test-example

## Note

split-test assumes test report has additional attribute `file` or `filepath`.

```xml
<testsuites>
<testsuite>
<testcase classname="spec.0_spec" name="0 is expected to eq 0" file="./spec/0_spec.rb" time="0.000373"></testcase>
</testsuite>
</testsuites>
```

To be exact it isn't JUnit standard.
