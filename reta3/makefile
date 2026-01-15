CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -O3 -I.
LDFLAGS = 
LIBS = -lpthread

TARGET = reta
SRCS = main.cpp error.cpp types.cpp utils.cpp i18n.cpp tables.cpp \
       tables_output.cpp tables_combi.cpp tables_prepare.cpp tables_maintable.cpp \
       tables_concat.cpp program.cpp
OBJS = $(SRCS:.cpp=.o)

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS) $(LIBS)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(TARGET) $(OBJS)

.PHONY: all clean
