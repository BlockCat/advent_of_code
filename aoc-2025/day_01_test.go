package main

import (
	"testing"
)

func TestKnobAdd50(t *testing.T) {
	var knob = newKnob()

	var r = knob.right(50)

	if r != 1 {
		t.Error("Expected to be 1")
	}

	if knob.value != 0 {
		t.Error("knob expected to be 0")
	}
}

func TestKnobAdd49(t *testing.T) {
	var knob = newKnob()

	var r = knob.right(49)

	if r != 0 {
		t.Error("Expected to be 0")
	}

	if knob.value != 99 {
		t.Error("knob expected to be 99")
	}
}

func TestKnobAdd51(t *testing.T) {
	var knob = newKnob()

	var r = knob.right(51)

	if r != 1 {
		t.Error("Expected to be 1")
	}

	if knob.value != 1 {
		t.Error("knob expected to be 1")
	}
}

func TestKnobAdd150(t *testing.T) {
	var knob = newKnob()

	var r = knob.right(150)

	if r != 2 {
		t.Error("Expected to be 2")
	}

	if knob.value != 0 {
		t.Error("knob expected to be 0")
	}
}
func TestKnobAdd151(t *testing.T) {
	var knob = newKnob()

	var r = knob.right(151)

	if r != 2 {
		t.Error("Expected to be 2")
	}

	if knob.value != 1 {
		t.Error("knob expected to be 1")
	}
}

func TestKnobRemove50(t *testing.T) {
	var knob = newKnob()

	var r = knob.left(50)

	if r != 1 {
		t.Error("Expected to be 1")
	}

	if knob.value != 0 {
		t.Error("knob expected to be 0")
	}
}

func TestKnobRemove51(t *testing.T) {
	var knob = newKnob()
	testMinus(t, knob, 51, 1, 99)
}

func TestKnobRemove49(t *testing.T) {
	var knob = newKnob()
	testMinus(t, knob, 49, 0, 1)
}

func TestKnobRemove150(t *testing.T) {
	var knob = newKnob()
	testMinus(t, knob, 150, 2, 0)
}

func TestKnobRemove151(t *testing.T) {
	var knob = newKnob()

	testMinus(t, knob, 50, 2, 99)
}

func TestKnobRemove149(t *testing.T) {
	var knob = newKnob()

	testMinus(t, knob, 49, 0, 1)
}

func TestKnob0Remove30(t *testing.T) {
	var knob = newKnob()
	knob.value = 0
	testMinus(t, knob, 30, 0, 70)
}
func TestKnob0Remove130(t *testing.T) {
	var knob = newKnob()
	knob.value = 0
	testMinus(t, knob, 130, 1, 70)
}

func testMinus(t *testing.T, knob *knob, m int, expectedR int, expectedValue int) {
	var r = knob.left(m)

	if r != expectedR {
		t.Errorf("Expected r: %d to be %d", r, expectedR)
	}

	if knob.value != expectedValue {
		t.Errorf("Expected knob.value: %d to be %d", knob.value, expectedValue)
	}
}
