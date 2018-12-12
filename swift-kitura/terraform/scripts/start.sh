#! /usr/bin/env bash

pkill swift
cd .build/release
./swift-kitura
cd -
