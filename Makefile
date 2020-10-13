release-all: release-log release-numeric 
release-log:
	cd log; make build-release; cd -;
release-numeric:
	cd numeric; make build-release; cd -;
