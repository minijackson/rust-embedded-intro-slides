#include <iostream>
#include <variant>
#include <vector>

// Questions:
//   - What is wrong with these functions?
//   - Does it compile? With warnings?
//   - Does it crash?
//   - What does Valgrind say?
//   - What do the sanitizers say?

void fn1()
{
	std::vector<int> my_vec{1, 2, 3};

	const auto& my_ref = my_vec[0];
	my_vec.clear();
	std::cout << my_ref << std::endl;
}

void fn2()
{
	std::vector<int> my_vec{1, 2, 3};

	const auto& my_ref = my_vec[0];
	my_vec.clear();
	my_vec.shrink_to_fit();
	std::cout << my_ref << std::endl;
}

void fn3()
{
	std::vector<std::variant<bool, int>> my_vec{true, 2, false};

	const auto& my_ref = std::get<bool>(my_vec[0]);
	my_vec[0] = 42; // Change a bool variant to an integer
	std::cout << std::boolalpha << my_ref << std::endl;
	// Actually very funny if you remove the std::boolalpha
}

// compile with:
// `-Wall -Wextra -std=c++17 -fsanitize=undefined,address,leak`
// Can't enable thread sanitizer, as it is incompatible with address
int main()
{
	//fn1();
	fn2();
	//fn3();
}
