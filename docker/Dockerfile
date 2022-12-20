FROM ubuntu:latest

RUN mkdir /home/scripts
COPY ./scripts /home/scripts
RUN cd /home && scripts/install/all/mint.sh
RUN git config --global --add safe.directory '*'
CMD bash
