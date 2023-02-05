FROM couchbase/centos7-systemd

RUN yum install make automake gcc gcc-c++ kernel-devel -y 

CMD /bin/bash
