#!/bin/bash
export $(cat .env | xargs)
cargo build --release
	
