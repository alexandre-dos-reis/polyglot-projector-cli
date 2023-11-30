package lib_test

import (
	"reflect"
	"testing"

	"github.com/alexandre-dos-reis/polyglot-projector-cli/golang/pkg/lib"
)

func getOtps(args []string) *lib.Opts {
	return &lib.Opts{
		Args:   args,
		Config: "",
		Pwd:    "",
	}
}

func testConfig(t *testing.T, args []string, expectedArgs []string, operation lib.Operation) {
	opts := getOtps(args)
	config, err := lib.NewConfig(opts)

	if err != nil {
		t.Errorf("expected to get no error %v", err)
	}

	if !reflect.DeepEqual(expectedArgs, config.Args) {
		t.Errorf("expect args to be %+v  ut got %+v", expectedArgs, config.Args)
	}

	if config.Operation != operation {
		t.Errorf("Operation expect was %v but got %v", expectedArgs, config.Operation)
	}
}

func TestConfigPrint(t *testing.T) {
	testConfig(t, []string{}, []string{}, lib.Print)
}

func TestConfigKey(t *testing.T) {
	testConfig(t, []string{"foo"}, []string{"foo"}, lib.Print)
}

func TestConfigAdd(t *testing.T) {
	testConfig(t, []string{"add", "foo", "bar"}, []string{"foo", "bar"}, lib.Add)
}

func TestConfigRemove(t *testing.T) {
	testConfig(t, []string{"rm", "foo"}, []string{"foo"}, lib.Remove)
}
