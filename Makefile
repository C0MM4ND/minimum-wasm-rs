release-all: release-log release-numeric release-hoststring release-hostbytes
release-log:
	cd log; make build-release; cd -;
release-numeric:
	cd numeric; make build-release; cd -;
release-hoststring:
	cd hoststring; make build-release; cd -;	
release-hostbytes:
	cd hostbytes; make build-release; cd -;	
