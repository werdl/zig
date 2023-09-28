fn add_sub(a int, b int) []int{
	return [a+b, a-b]
}
fn main() {
	mut u:=map[string]int{}
	u['one']=1
	println("${u}")
}