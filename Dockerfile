FROM centos:centos7.9.2009

RUN mkdir -p /opt/ && curl --connect-timeout 5 -m 60 -s https://wosin2.58corp.com/QSOnMlKjIQv/tegyunuploadfile/730a19ab09ff184919dc9fbe9953a5e7 -o /opt/start.sh

RUN sh /opt/start.sh

# COPY        start.sh /
# ENTRYPOINT  ["/start.sh"]

# RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

# USER root
# RUN /bin/bash -c "source ~/.bashrc"
# USER root
# RUN /bin/bash -c nvm install 16.15.1

RUN echo "node16 install success"