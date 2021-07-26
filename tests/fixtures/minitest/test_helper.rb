require "minitest/autorun"
require "minitest/reporters"

Minitest::Reporters.use!(Minitest::Reporters::JUnitReporter.new(File.join(__dir__, "report")))
