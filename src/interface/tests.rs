//! Should test the interface overall to make sure it does what is expected of it.

describe! interface {

	before_each {
		// Start up a test.
		let mut stainless = true;
	}

	it "makes organizing tests easy" {
		// Do the test.
		assert!(stainless);
	}

	after_each {
		// End the test.
		stainless = false;
	}

	bench "something simple" (bencher) {
		bencher.iter(|| 2 * 2)
	}

	describe! nesting {

		before_each {
		  let mut inner_stainless = true;
		}

		after_each {
		  inner_stainless = false;
		}

		it "makes it simple to categorize tests" {
			// It even generates submodules!
			assert_eq!(2, 2);
		}
	}

}
