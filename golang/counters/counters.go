package counters

type alertCounter int

// New created exported function
// type alertCounter
func New(value int) alertCounter {
	return alertCounter(value)
}
