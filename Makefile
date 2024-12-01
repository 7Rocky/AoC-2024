test:
	@for d in $(shell ls -d ${PWD}/*/ | grep day_); do \
		cd $${d} && echo '\nTesting:' && pwd && rustc main_test.rs && rustc main.rs && ./main_test time > output.txt && ./main >> output.txt && ./main_test && rm main main_test output.txt; \
	done

.PHONY: test
