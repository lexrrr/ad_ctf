from imp import max_apart, polygon_area, inside_polygon
from flask import Flask, request, jsonify
import numpy as np
import logging

app = Flask(__name__)

# Set up logging
logging.basicConfig(level=logging.DEBUG)

@app.route("/check-point", methods=["POST"])
def create_polygon():
    data = request.get_json()
    logging.debug(f"Received data: {data}")

    searchpoints = data.get("searchpoints")
    check_point = data.get("check_point")

    if not searchpoints or not check_point:
        return jsonify({"error": "Invalid input"}), 400

    # Round the coordinates to avoid precision issues
    def round_coordinates(coords, precision=6):
        return [round(coord, precision) for coord in coords]

    vertices = np.array([round_coordinates(point) for point in searchpoints])
    check_point = round_coordinates(check_point)

    try:
        if not np.array_equal(vertices[0], vertices[-1]):
            vertices = np.vstack([vertices, vertices[0]])

    except Exception as e:
        logging.error(f"Error closing polygon: {e}", exc_info=True)
        return jsonify({"error": 3, "message": str(e), "debug": vertices.tolist()}), 500

    try:
        area = polygon_area(vertices)
        logging.debug(f"Polygon area: {area}")
        if area > 0.7:
            return jsonify({"error": 1.1}), 200
    except Exception as e:
        logging.error(f"Error calculating polygon area: {e}", exc_info=True)
        return jsonify({"error": 4, "message": str(e), "debug": vertices.tolist()}), 500

    try:
        if not max_apart(vertices, 15):  # change to lower (atm 35km) after testing
            return jsonify({"error": 1.2}), 200
    except Exception as e:
        logging.error(f"Error checking max_apart: {e}", exc_info=True)
        return jsonify({"error": 5, "message": str(e), "debug": vertices.tolist()}), 500

    try:
        ret = inside_polygon(check_point, vertices)
        return jsonify({"close": f"{1 if ret else 0}"}), 200
    except Exception as e:
        logging.error(f"Error checking inside_polygon: {e}", exc_info=True)
        return jsonify(
            {
                "error": 2,
                "message": str(e),
                "debug": vertices.tolist(),
                "debug2": check_point,
            }
        ), 200


