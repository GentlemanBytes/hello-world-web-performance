package com.gentlemambytes.javaspark;

import static spark.Spark.*;

import java.util.HashMap;
import java.util.Map;

import spark.Request;
import spark.Response;
import spark.ModelAndView;
import spark.template.velocity.VelocityTemplateEngine;

/**
 * Hello world!
 *
 */
public class App 
{
    public static void main(final String[] args) {

        get("/", (request, response) -> {
            Map<String, Object> model = new HashMap<>();
            model.put("what", "world");

            // The vm files are located under the resources directory
            return new ModelAndView(model, "templates/index.vm");
        }, new VelocityTemplateEngine());

    }
}