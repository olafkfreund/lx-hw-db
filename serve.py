#!/usr/bin/env python3
"""
Simple HTTP server for local development of the Linux Hardware Compatibility Database
"""
import http.server
import socketserver
import os
import sys
from urllib.parse import urlparse, parse_qs
import json
import mimetypes

class LXHWDBHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=os.path.dirname(os.path.abspath(__file__)), **kwargs)
    
    def end_headers(self):
        # Add CORS headers for local development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()
    
    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()
    
    def do_GET(self):
        parsed_path = urlparse(self.path)
        
        # Handle API endpoints
        if parsed_path.path.startswith('/api/'):
            return self.handle_api_request(parsed_path)
        
        # Serve static files
        return super().do_GET()
    
    def do_POST(self):
        parsed_path = urlparse(self.path)
        
        # Handle API POST requests
        if parsed_path.path.startswith('/api/'):
            return self.handle_api_post(parsed_path)
        
        self.send_error(404, "Not Found")
    
    def handle_api_request(self, parsed_path):
        """Handle API GET requests"""
        path = parsed_path.path
        
        if path == '/api/hardware':
            return self.serve_hardware_database()
        elif path == '/api/tips':
            return self.serve_configuration_tips()
        elif path == '/api/statistics':
            return self.serve_statistics()
        else:
            self.send_error(404, f"API endpoint not found: {path}")
    
    def handle_api_post(self, parsed_path):
        """Handle API POST requests for data submission"""
        path = parsed_path.path
        content_length = int(self.headers.get('content-length', 0))
        
        if content_length > 10 * 1024 * 1024:  # 10MB limit
            self.send_error(413, "Request too large")
            return
        
        try:
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data.decode('utf-8'))
            
            if path == '/api/hardware/submit':
                return self.handle_hardware_submission(data)
            elif path == '/api/tips/submit':
                return self.handle_tip_submission(data)
            else:
                self.send_error(404, f"API endpoint not found: {path}")
                
        except json.JSONDecodeError:
            self.send_error(400, "Invalid JSON data")
        except Exception as e:
            print(f"Error handling POST request: {e}")
            self.send_error(500, "Internal server error")
    
    def serve_hardware_database(self):
        """Serve the hardware database"""
        try:
            with open('data/hardware-database.json', 'r') as f:
                data = json.load(f)
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps(data).encode('utf-8'))
            
        except FileNotFoundError:
            self.send_error(404, "Hardware database not found")
        except Exception as e:
            print(f"Error serving hardware database: {e}")
            self.send_error(500, "Error loading hardware database")
    
    def serve_configuration_tips(self):
        """Serve the configuration tips database"""
        try:
            with open('data/configuration-tips.json', 'r') as f:
                data = json.load(f)
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps(data).encode('utf-8'))
            
        except FileNotFoundError:
            self.send_error(404, "Configuration tips database not found")
        except Exception as e:
            print(f"Error serving configuration tips: {e}")
            self.send_error(500, "Error loading configuration tips")
    
    def serve_statistics(self):
        """Serve combined statistics"""
        try:
            # Load both databases
            with open('data/hardware-database.json', 'r') as f:
                hardware_data = json.load(f)
            
            with open('data/configuration-tips.json', 'r') as f:
                tips_data = json.load(f)
            
            # Combine statistics
            combined_stats = {
                'hardware': hardware_data.get('statistics', {}),
                'tips': tips_data.get('statistics', {}),
                'combined': {
                    'total_entries': hardware_data['statistics']['total_hardware'] + tips_data['statistics']['total_tips'],
                    'last_updated': max(
                        hardware_data['statistics']['last_updated'],
                        tips_data['statistics']['last_updated']
                    )
                }
            }
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps(combined_stats).encode('utf-8'))
            
        except Exception as e:
            print(f"Error serving statistics: {e}")
            self.send_error(500, "Error loading statistics")
    
    def handle_hardware_submission(self, data):
        """Handle hardware report submission"""
        print(f"Received hardware submission: {data.get('id', 'unknown')}")
        
        # In a real implementation, this would:
        # 1. Validate the hardware report data
        # 2. Add it to the database
        # 3. Generate indices
        # 4. Return success/failure
        
        # For now, just acknowledge receipt
        response = {
            'status': 'success',
            'message': 'Hardware report received and will be processed',
            'id': data.get('id', f'hw_{int(os.time())}')
        }
        
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        self.wfile.write(json.dumps(response).encode('utf-8'))
    
    def handle_tip_submission(self, data):
        """Handle configuration tip submission"""
        print(f"Received configuration tip: {data.get('title', 'untitled')}")
        
        # In a real implementation, this would:
        # 1. Validate the tip data
        # 2. Check for spam/malicious content
        # 3. Add to moderation queue
        # 4. Return success/failure
        
        response = {
            'status': 'success', 
            'message': 'Configuration tip submitted for moderation',
            'id': data.get('id', f'tip_{int(os.time())}')
        }
        
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        self.wfile.write(json.dumps(response).encode('utf-8'))

def main():
    port = 8000
    
    # Check if port is provided as argument
    if len(sys.argv) > 1:
        try:
            port = int(sys.argv[1])
        except ValueError:
            print(f"Invalid port number: {sys.argv[1]}")
            return
    
    # Check if data files exist
    data_files = ['data/hardware-database.json', 'data/configuration-tips.json']
    missing_files = [f for f in data_files if not os.path.exists(f)]
    
    if missing_files:
        print("⚠️  Warning: Missing data files:")
        for f in missing_files:
            print(f"   - {f}")
        print("\nThe website will still work, but no hardware data will be available.")
        print("Run the hardware detection tool to populate the database.")
        print()
    
    try:
        with socketserver.TCPServer(("", port), LXHWDBHandler) as httpd:
            print(f"🌐 Linux Hardware Compatibility Database Server")
            print(f"📡 Server starting on http://localhost:{port}")
            print(f"📁 Serving from: {os.getcwd()}")
            print()
            print(f"🔍 Hardware Database: {'✅ Found' if os.path.exists('data/hardware-database.json') else '❌ Missing'}")
            print(f"💡 Configuration Tips: {'✅ Found' if os.path.exists('data/configuration-tips.json') else '❌ Missing'}")
            print()
            print("📖 Available endpoints:")
            print("   - http://localhost:{port}/              (Main website)")
            print("   - http://localhost:{port}/api/hardware   (Hardware database API)")
            print("   - http://localhost:{port}/api/tips       (Configuration tips API)")
            print("   - http://localhost:{port}/api/statistics (Combined statistics)")
            print()
            print("⚡ To populate with real data, run the hardware detection tool:")
            print("   cargo run --bin lx-hw-detect -- --output web/data/my-hardware.json")
            print()
            print("🛑 Press Ctrl+C to stop the server")
            print("=" * 60)
            
            httpd.serve_forever()
            
    except KeyboardInterrupt:
        print("\n👋 Server stopped by user")
    except OSError as e:
        if e.errno == 48:  # Address already in use
            print(f"❌ Port {port} is already in use")
            print(f"   Try a different port: python3 serve.py {port + 1}")
        else:
            print(f"❌ Server error: {e}")

if __name__ == "__main__":
    main()