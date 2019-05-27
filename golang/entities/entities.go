package entities

type User struct {
	Name  string
	Email string
}

type Admin struct {
	user
	Rights int
}
