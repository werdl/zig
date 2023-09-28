fn add_sub(a int, b int) []int{
	return [a+b, a-b]
}
	struct Fooo {
		fooval string
	}
	struct Baro {
		barval string
	}
	type Foobaro = Fooo|Baro;
fn main() {



	x:=Foobaro(Fooo{"foo"})
	if x is Fooo {
		println(x)
	}
}